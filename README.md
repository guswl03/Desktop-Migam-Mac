<p align="center">
  <img src="images/app/icon-source.png" width="128" alt="migam desktop 감자봇 아이콘">
</p>

<h1 align="center">migam desktop for macOS</h1>

<p align="center">
  Mac 바탕화면을 돌아다니며 집중을 도와주는 감자봇 데스크톱 펫
</p>

<p align="center">
  <a href="https://github.com/guswl03/Desktop-Migam-Mac/releases/download/v0.1.1/migam.desktop_0.1.1_aarch64.dmg"><strong>Apple Silicon용 DMG 다운로드</strong></a>
  ·
  <a href="https://github.com/guswl03/Desktop-Migam-Mac/releases/tag/v0.1.1">v0.1.1 릴리즈 보기</a>
</p>

> [!NOTE]
> 현재 배포판은 **Apple Silicon(M2 이상) 및 macOS 14 Sonoma 이상**을 지원합니다. Intel Mac에서는 실행되지 않습니다.

## 바로 설치하기

1. [migam desktop DMG 다운로드](https://github.com/guswl03/Desktop-Migam-Mac/releases/download/v0.1.1/migam.desktop_0.1.1_aarch64.dmg)를 누릅니다.
2. 다운로드한 `migam.desktop_0.1.1_aarch64.dmg`를 엽니다.
3. `migam desktop`을 **Applications** 폴더로 드래그합니다.
4. Finder의 **응용 프로그램**에서 `migam desktop`을 실행합니다.

다운로드가 시작되지 않으면 [v0.1.1 릴리즈 페이지](https://github.com/guswl03/Desktop-Migam-Mac/releases/tag/v0.1.1)의 **Assets**에서 DMG를 직접 내려받으세요.

### macOS가 앱 실행을 차단한다면

현재 앱은 Apple Developer ID로 서명하거나 공증하지 않은 빌드입니다.

1. Finder의 **응용 프로그램**에서 `migam desktop`을 Control-클릭하고 **열기**를 선택합니다.
2. 그래도 차단되면 **시스템 설정 → 개인정보 보호 및 보안**으로 이동합니다.
3. 차단 안내 옆의 **확인 없이 열기** 또는 **그래도 열기**를 누릅니다.
4. macOS 암호나 Touch ID로 승인한 뒤 다시 실행합니다.

출처를 확인할 수 없는 다른 파일에는 이 절차를 사용하지 마세요. 이 저장소의 릴리즈 페이지에서 받은 DMG인지 먼저 확인하세요.

### Accessibility 권한 허용

방해 창 감지·최소화와 창 위 이동 기능을 사용하려면 손쉬운 사용 권한이 필요합니다.

1. **시스템 설정 → 개인정보 보호 및 보안 → 손쉬운 사용**을 엽니다.
2. `migam desktop`을 활성화합니다.
3. 앱을 완전히 종료한 뒤 다시 실행합니다.

권한을 허용하지 않아도 데스크톱 펫, 타이머, 할 일과 GAMCHA 기능은 사용할 수 있습니다. 방해 창 개입은 기본적으로 꺼져 있습니다.

### 처음 실행했다면

- 감자봇을 **마우스 왼쪽 버튼으로 드래그**해 옮길 수 있습니다. 빠르게 놓으면 던져집니다.
- 감자봇을 **마우스 오른쪽 버튼으로 클릭**하면 타이머, 할 일, GAMCHA와 설정이 열립니다.
- 메뉴 막대의 감자봇 아이콘에서 창 표시, 긴급 중지, 다시 시작과 종료를 선택할 수 있습니다.
- 모든 동작을 즉시 멈추려면 **`Command+Shift+F12`**를 누르세요.

### 삭제하기

앱을 종료한 뒤 Finder의 **응용 프로그램**에서 `migam desktop`을 휴지통으로 이동합니다.

## 주요 기능

- 투명·항상 위 펫 창과 Retina·다중 모니터 작업 영역 이동
- 클릭, 드래그, 던지기, 착지·충돌·창 오르기 애니메이션
- CPU·메모리 사용량에 반응하는 이동
- 뽀모도로 타이머와 할 일 집중 연결
- 집중 완료 보상 티켓, GAMCHA와 156종 코스튬 인벤토리
- 사진 배달과 1% 희귀 이벤트
- Accessibility 기반 활성 창 감지와 대상 재검증 후 안전한 최소화
- 설정·할 일·GAMCHA 로컬 저장 및 손상 데이터 보존·복구

> [!IMPORTANT]
> 방해 창 개입은 기본적으로 꺼져 있습니다. 사용자가 규칙을 만들고 명시적으로 활성화해야 동작하며, 실제 창 제목이나 앱 경로는 저장하거나 로그로 남기지 않습니다. 권한이 없거나 대상을 확실히 판정할 수 없으면 창을 조작하지 않습니다.

## 요구 사항과 알려진 제한

- Apple Silicon M2 이상
- macOS 14 Sonoma 이상
- 창 개입 기능 사용 시 Accessibility 권한
- 자동 업데이트, Apple Developer ID 서명과 공증은 아직 지원하지 않습니다.
- Intel Mac, macOS 13 이하, Windows와 Linux는 지원하지 않습니다.
- Retina 배율, 외장 모니터, Dock 위치와 특수 최상단 창은 환경에 따라 동작 차이가 있을 수 있습니다.

## 로컬 데이터와 개인정보

설정·할 일·GAMCHA 진행도는 macOS 앱 데이터 디렉터리의 `settings.json`, `todo.json`, `gamcha.json`에 저장됩니다. 손상된 파일은 이름에 복구 시각을 붙여 별도로 보관하고 안전한 기본값으로 복구합니다.

- 계정, 클라우드 동기화와 분석 기능 없음
- 실제 창 제목과 앱 경로 저장·로그 금지
- Finder, Dock, 시스템 설정, 로그인·보안 관련 창과 판정 불가 창은 최소화하지 않음

## 개발하기

### 준비물

- Apple Silicon Mac과 macOS 14 이상
- Node.js 22 이상과 npm
- Rust stable 및 `aarch64-apple-darwin` target
- Xcode Command Line Tools

### 실행

```bash
npm ci
npm run tauri -- dev
```

### 검증

```bash
npm run typecheck
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin
cargo clippy --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --all-targets -- -D warnings
```

### Apple Silicon DMG 만들기

```bash
npm run tauri build -- --target aarch64-apple-darwin --bundles app,dmg
```

생성된 앱과 DMG는 `src-tauri/target/aarch64-apple-darwin/release/bundle/` 아래에 저장됩니다.

## 현재 검증 상태

- 프런트엔드 테스트 48개 통과
- Rust 테스트 46개 통과
- TypeScript 검사, Vite production build, rustfmt와 Clippy 통과
- GitHub Actions의 Apple Silicon macOS 빌드와 DMG 생성 통과
- 제공된 DMG의 실제 Mac 실행, 코드 서명·공증, 장시간 안정성은 별도 수동 확인 필요

상세한 macOS 개발·검증 절차는 [macOS 개발 가이드](docs/19-macos-development.md)를 참고하세요.