# migam desktop macOS 전용 이식 설계

## 1. 목표

기존 Windows용 `migam desktop`을 Apple Silicon 전용 macOS 애플리케이션으로 이식한다. 펫 애니메이션, 드래그·던지기, 뽀모도로, 투두리스트, GAMCHA, 사진 배달, 설정 저장, 시스템 사용률 반응을 유지하고, 집중 중 방해 앱 감지와 안전한 창 최소화를 macOS Accessibility API로 다시 구현한다.

## 2. 지원 범위

- CPU: Apple Silicon M2 이상
- 운영체제: macOS 14 Sonoma 이상
- Rust 빌드 타깃: `aarch64-apple-darwin`
- 배포 산출물: `.app`, 개발·테스트용 DMG
- UI 기술: Tauri 2, Vite, 순수 TypeScript, DOM API
- 데이터: 기존 로컬 JSON 형식과 도메인 규칙 유지
- 기본 동작: 창 개입은 off

다음은 이번 이식 범위에서 제외한다.

- Intel Mac과 Universal Binary
- macOS 13 이하
- Windows 빌드 호환성 유지
- Mac App Store 배포
- 자동 업데이트
- Developer ID 인증서 발급과 실제 공증 수행
- Linux 지원

## 3. 성공 조건

1. macOS 14 이상의 M2 또는 M3 Mac에서 앱이 실행되고 투명 펫 창과 메뉴 막대 트레이가 정상 표시된다.
2. 펫의 이동, 다중 모니터 작업 영역 경계, 드래그, 던지기, 착지와 기존 애니메이션이 정상 동작한다.
3. 뽀모도로, 투두리스트, GAMCHA, 코스튬, 사진 배달과 설정 저장·손상 복구가 기존 계약대로 동작한다.
4. CPU·메모리 사용률을 읽어 펫의 이동·애니메이션 단계를 갱신한다.
5. Accessibility 권한이 있을 때 활성 앱과 창을 감지하고, 방해 규칙과 보호 규칙을 적용한 뒤 경고와 재검증을 거쳐 동일한 대상 창만 최소화한다.
6. Accessibility 권한이 없거나 거부되면 앱은 계속 실행되고 창 감지·최소화만 안전하게 비활성화된다.
7. 긴급 중지, 개입 기본 off, 보호 대상 판정과 대상 재검증은 제거하거나 약화하지 않는다.
8. TypeScript 검사, 프런트 테스트·빌드, Rust 테스트·포맷·Clippy와 macOS 네이티브 빌드가 통과한다.

## 4. 아키텍처

기존 `domain`과 `application` 계층은 운영체제와 무관한 진실 공급원으로 유지한다. Windows 전용 `infrastructure/windows`는 제거하고 `infrastructure/macos`가 기존 포트인 `ForegroundWindowSource`와 `WindowMinimizer`를 구현한다. 시스템 사용률 수집도 같은 macOS 인프라 계층에 둔다.

```text
TypeScript windows and views
          |
    Tauri commands/events
          |
domain + application services
          |
macOS adapters
  - Accessibility permission
  - foreground window inspection
  - safe window minimization
  - CPU and memory metrics
```

플랫폼 구현을 프런트에 노출하지 않는다. 프런트는 권한 상태와 기능 사용 가능 여부만 계약 필드로 받고, 기존 명령과 이벤트 흐름을 계속 사용한다.

## 5. macOS 모듈 경계

### 5.1 Accessibility 권한

`accessibility.rs`는 `AXIsProcessTrustedWithOptions`를 사용한다.

- 상태 조회는 시스템 프롬프트를 띄우지 않는다.
- 사용자가 방해 앱 개입을 켤 때만 프롬프트 옵션을 사용한다.
- 프롬프트는 비동기이므로 즉시 성공으로 간주하지 않는다.
- 권한 상태는 `granted`, `denied`, `not_determined`, `unavailable` 중 하나로 프런트에 전달한다.
- 권한이 없으면 감시 루프는 다른 앱을 조사하거나 최소화하지 않는다.

### 5.2 활성 앱과 창 조회

`foreground_window.rs`는 macOS의 전면 앱 PID를 얻고 해당 프로세스의 `AXUIElement`에서 포커스된 창을 조회한다. 스냅샷에는 다음 값을 담는다.

- PID
- 앱 번들 ID
- 실행 파일 또는 표시 이름
- 창 제목
- 창 위치와 크기
- 최소화 여부
- 전체 화면 여부
- 현재 모니터의 좌측 좌표

Windows의 `HWND` 값은 macOS에서 의미가 없으므로 도메인 계약의 `window_id`를 불투명한 대상 키로 바꾼다. 대상 키는 PID, 창 제목, 위치와 크기에서 만들어 한 번의 개입 수명 안에서만 사용하며 영구 저장하지 않는다.

### 5.3 대상 재검증과 최소화

기존 application 계층의 경고·유예·쿨다운 흐름을 유지한다. 경고 애니메이션이 끝난 직후 활성 앱과 포커스 창을 다시 읽고 다음 조건을 모두 만족할 때만 최소화한다.

- Accessibility 권한이 여전히 허용됨
- PID와 번들 ID가 최초 스냅샷과 같음
- 창 제목과 기하 정보가 허용 오차 안에서 같음
- 창이 표시 중이며 이미 최소화되지 않음
- 전체 화면 또는 보호 대상이 아님
- 긴급 중지가 활성화되지 않음

최소화는 포커스 창의 `kAXMinimizedAttribute`가 설정 가능한지 확인한 뒤 `true`로 설정한다. 속성이 없거나 설정 불가능하거나 AX 호출이 실패하면 실패로 기록하고 아무 대체 조작도 하지 않는다. 앱 숨기기, 강제 종료, 키 입력 합성, AppleScript 실행은 사용하지 않는다.

### 5.4 보호 대상

자기 앱과 다음 시스템 구성 요소는 기본 보호한다.

- Finder
- Dock
- System Settings
- loginwindow
- Control Center
- Notification Center
- 암호, 인증, 권한 승인 또는 보안 관련 제목을 가진 창

번들 ID를 우선 사용하고, 얻지 못한 경우 프로세스명과 창 제목의 보수적인 규칙으로 판정한다. 판정이 불확실하면 보호된 것으로 취급한다.

### 5.5 시스템 사용률

CPU와 메모리 측정은 Windows API를 제거하고 macOS에서 검증된 Rust 시스템 정보 API로 교체한다. 외부 crate를 선택할 때는 Apple Silicon과 macOS 14 지원, CPU 갱신 간격, 메모리 단위와 유지보수 상태를 확인한다. 기존 0~100 정수 계약과 750ms 캐시는 유지해 프런트 애니메이션을 바꾸지 않는다.

## 6. Tauri와 프런트 변경

- `tauri.conf.json`의 공통 창 정의는 유지하되 Windows NSIS 설정을 제거한다.
- `tauri.macos.conf.json`에 `minimumSystemVersion: "14.0"`, macOS 아이콘과 DMG 설정을 둔다.
- 앱 아이콘은 `.icns`를 생성해 번들에 포함한다.
- Windows 전용 GUI subsystem 속성과 WebView2 안내를 제거한다.
- 메뉴 막대 트레이는 Tauri tray API를 유지한다.
- 긴급 단축키는 `Command+Shift+F12`로 바꾼다.
- 설정 화면에 Accessibility 권한 상태, 권한 요청 버튼과 시스템 설정 안내를 표시한다.
- macOS에서 의미가 없는 `.exe` 예시와 Windows 전용 기본 방해 규칙은 번들 ID 또는 macOS 앱 이름 기준으로 바꾼다.
- 기존 WinDbg풍 시각 디자인은 제품 디자인으로 유지하되 Windows 기능을 암시하는 설명 문구는 macOS 용어로 교체한다.

## 7. 데이터 흐름

### 일반 실행

1. 앱 시작 시 설정, 타이머, 투두와 GAMCHA 데이터를 복구한다.
2. Accessibility 권한은 프롬프트 없이 조회한다.
3. 트레이와 모든 Tauri 창을 생성한다.
4. CPU·메모리 수집과 타이머 ticker를 시작한다.
5. 창 개입이 off이거나 권한이 없으면 포그라운드 창 조회를 생략한다.

### 집중 개입

1. 집중 상태이며 개입이 on이고 권한이 있을 때 활성 창을 읽는다.
2. 방해 규칙과 보호 규칙을 평가한다.
3. 유예 시간을 만족하면 경고 요청을 생성한다.
4. 경고가 끝나면 현재 활성 창을 다시 읽는다.
5. 같은 대상이며 모든 안전 조건을 만족할 때만 AX 최소화를 실행한다.
6. 성공 여부와 관계없이 쿨다운과 UI 상태를 일관되게 정리한다.

## 8. 오류 처리와 안전 정책

- 권한 부족: 기능 비활성화 상태를 표시하고 나머지 앱은 계속 실행한다.
- AX 호출 실패 또는 응답 지연: 해당 tick을 건너뛰고 최소화하지 않는다.
- 활성 창 변경: 진행 중인 개입을 취소한다.
- 번들 ID, 제목 또는 기하 정보 누락: 보호 대상으로 취급한다.
- 전체 화면 앱: 기본적으로 최소화하지 않는다.
- 최소화 불가능한 창: 실패를 반환하고 추가 조작을 하지 않는다.
- 앱 종료 또는 긴급 중지: 진행 중 경고와 감시 상태를 즉시 취소한다.
- 로그: PID, 번들 ID와 오류 종류만 진단에 사용하며 전체 창 제목은 기본 로그에 남기지 않는다.

## 9. 빌드와 배포

개발 단계에서는 ad-hoc 서명으로 Apple Silicon `.app`과 DMG를 만든다. 외부 사용자 배포는 Developer ID Application 인증서와 Apple 공증이 준비된 뒤 별도 릴리스 단계로 수행한다. 인증서, Apple ID, API 키와 비밀번호는 저장소에 저장하지 않고 CI 비밀값으로만 주입한다.

macOS 네이티브 산출물은 Windows에서 완전히 검증할 수 없으므로 다음 두 경로를 사용한다.

1. Windows에서 플랫폼 독립 단위 테스트, TypeScript 검사와 Vite 빌드 실행
2. GitHub Actions의 macOS Apple Silicon 실행기 또는 실제 M2/M3 Mac에서 Rust 테스트, Clippy, Tauri 빌드와 수동 검증 실행

원격 릴리스 게시, 서명과 공증은 별도의 명시적 승인 없이는 수행하지 않는다.

## 10. 테스트 전략

### 자동 테스트

- 기존 도메인·application 테스트를 유지한다.
- 권한 상태별 감시 활성화 여부를 테스트한다.
- PID·번들 ID·제목·기하 정보가 바뀐 경우 재검증이 실패하는지 테스트한다.
- 보호 번들 ID와 보안 제목 규칙을 테스트한다.
- AX 어댑터는 trait 경계 뒤에 두고 성공, 권한 부족, 지원하지 않는 속성, 타임아웃을 fake로 테스트한다.
- 시스템 사용률 값이 0~100 범위를 유지하는지 테스트한다.
- macOS 설정 병합과 `aarch64-apple-darwin` 빌드를 CI에서 검증한다.

### 실제 Mac 수동 테스트

- 최초 권한 요청과 거부·허용·재실행 흐름
- Chrome, Safari, Discord 등 일반 앱의 감지와 최소화
- Finder, System Settings, 비밀번호·인증 창 보호
- 경고 중 앱 전환 시 개입 취소
- 전체 화면과 Stage Manager
- 다중 모니터, 서로 다른 배율과 메뉴 막대·Dock 위치
- 펫 투명도, 항상 위, 입력 통과 창과 드래그
- 메뉴 막대 트레이와 `Command+Shift+F12`
- `.app` 및 DMG 설치 후 실행

## 11. 구현 단계

1. 플랫폼 계약과 macOS 빌드 골격 정리
2. Accessibility 권한 서비스 구현
3. 활성 앱·창 조회와 보호 규칙 구현
4. 재검증 기반 최소화 구현
5. macOS CPU·메모리 수집 구현
6. 프런트 권한 상태와 macOS 문구 적용
7. 아이콘, `.app`, DMG와 Apple Silicon CI 구성
8. 실제 M2/M3 Mac 수동 검증과 결함 수정

각 단계는 실패하는 테스트를 먼저 추가하고 관련 자동 검증을 통과한 뒤 다음 단계로 진행한다.

## 12. 완료 조건과 남은 외부 의존성

코드 완료는 자동 테스트와 macOS 네이티브 빌드가 모두 통과한 상태를 뜻한다. 제품 완료는 실제 M2 또는 M3 Mac에서 Accessibility 권한, 다중 모니터, 창 최소화와 투명 창 동작을 수동 확인한 뒤에만 선언한다.

다음 항목은 구현만으로 해결할 수 없는 외부 의존성이다.

- 실제 Apple Silicon Mac 또는 macOS CI 실행 환경
- 외부 배포 시 Apple Developer 계정
- Developer ID 인증서와 공증용 비밀값
