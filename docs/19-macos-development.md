# macOS 개발·검증 가이드

## 지원 환경

- Apple Silicon M2 이상
- macOS 14 Sonoma 이상
- Xcode Command Line Tools
- Node.js 22
- Rust stable과 `aarch64-apple-darwin` 타깃

## 준비

```bash
xcode-select --install
rustup target add aarch64-apple-darwin
npm ci
```

## 개발 실행

```bash
npm run tauri dev
```

집중 보호 기능을 켜면 macOS가 손쉬운 사용 권한을 요청한다. 시스템 설정의 `개인정보 보호 및 보안 > 손쉬운 사용`에서 `migam desktop`을 허용한 후 앱으로 돌아와 권한 상태를 다시 확인한다. 권한이 없으면 펫, 타이머, 투두, GAMCHA는 계속 동작하고 다른 앱의 창 감지와 최소화만 비활성화된다.

## 자동 검증

```bash
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin
cargo clippy --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --all-targets -- -D warnings
```

## 앱·DMG 빌드

개발 검증용 ad-hoc 서명:

```bash
APPLE_SIGNING_IDENTITY="-" npm run tauri build -- --target aarch64-apple-darwin --bundles app,dmg
```

외부 배포에는 Developer ID Application 인증서와 Apple 공증이 필요하다. 인증서와 공증 비밀값은 저장소에 넣지 않는다.

## 실제 Mac 수동 검증

- 최초 손쉬운 사용 권한 요청, 거부, 허용과 재실행
- Safari, Chrome, Discord 등 일반 앱 감지와 최소화
- Finder, Dock, System Settings, 암호·인증 창 보호
- 경고 중 앱 전환 시 개입 취소
- 전체 화면과 Stage Manager
- 다중 모니터와 서로 다른 배율
- 메뉴 막대와 Dock을 제외한 펫 작업 영역
- 투명 창, 항상 위, 드래그, 던지기와 착지
- 메뉴 막대 트레이
- `Command+Shift+F12` 긴급 중지
- `.app` 직접 실행과 DMG 설치 실행

수동으로 실행하지 않은 항목은 통과로 기록하지 않는다.
