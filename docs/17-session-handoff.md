# 세션 인수인계

최종 갱신: 2026-08-23

이 문서는 다음 작업 세션이 가장 먼저 확인하는 단일 인수인계 기록이다. 작업을 끝낼 때마다 오래된 내용을 남기지 말고 현재 상태로 갱신한다.

## 현재 목표

기반 구축 단계(환경·앱 셸·설정 기반)를 검증 가능한 상태로 완료한다.

## 현재 상태

- Tauri 2 + Rust + Vite + 순수 TypeScript 프로젝트가 생성되어 있다.
- `pet`, `card`, `timer`, `settings` 창이 구성되어 있다.
- 트레이 메뉴에서 펫·타이머·설정 표시, 긴급 중지, 재시작, 종료를 요청할 수 있다.
- 설정 기본값·검증·schema v1→v2 migration이 있다.
- 설정 JSON 저장, 손상 파일 보관과 기본값 복구가 구현되어 있다.
- 설정 창에서 펫 크기와 뽀모도로 기본값을 편집하고 저장할 수 있다.
- 창 개입은 기본 off이고 규칙이 없으면 활성화할 수 없다.
- 전역 긴급 중지 단축키 `Ctrl+Shift+F12`가 연결되어 있으며 등록 실패 시 설정 화면에 경고한다.
- 트레이 또는 단축키 초기화 실패가 앱 전체를 종료시키지 않도록 격리되어 있다.
- Vite가 `src-tauri/target`과 `.tools`를 감시하지 않아 잠긴 Rust 실행 파일로 인한 EBUSY 종료를 피한다.
- Gamjabot 디자인 보드와 10개 상태·35개 투명 PNG 프레임, manifest, 재생성 스크립트가 추가되었다.
- 펫 창의 기본 placeholder 이미지는 Gamjabot idle 프레임을 사용한다.

## 이번 세션 변경 파일

- `AGENTS.md`
- `package.json`
- `src/contracts.ts`
- `src/main.ts`
- `src/styles.css`
- `src-tauri/src/lib.rs`
- `src-tauri/src/app_state.rs`
- `src-tauri/src/application/mod.rs`
- `src-tauri/src/application/settings_service.rs`
- `src-tauri/src/presentation/mod.rs`
- `src-tauri/src/presentation/commands.rs`
- `src-tauri/src/presentation/tray.rs`
- `src-tauri/src/domain/settings.rs`
- `src-tauri/src/domain/distraction.rs`
- `images/characters/gamjabot/design/`
- `images/characters/gamjabot/source/`
- `images/characters/gamjabot/pack/`
- `scripts/generate-gamjabot-pack.py`
- 관련 개발·진행 문서

## 검증 상태

- Node.js: `v24.18.0`
- npm: `11.16.0`
- 프로젝트 `.tools`에 Rust `1.98.0` stable MSVC가 설치되어 있다.
- `. .\scripts\use-project-rust.ps1`을 실행하면 현재 PowerShell에서 Rust/Cargo를 사용할 수 있다.
- npm 의존성 설치 완료.
- `npm run typecheck`, `npm run build`, `npm test` 통과. 프런트 테스트 파일은 아직 없다.
- Gamjabot 생성 검증 통과: 256×256 RGBA, 가시 팔레트 순수 검정·흰색, 10개 상태.
- Rust 테스트 16개, rustfmt, Clippy와 Tauri release 앱 빌드가 통과했다.
- Codex 샌드박스에서는 Windows GUI 창 생성이 access denied로 차단되어 실제 실행 수동 확인이 필요하다.

## 차단 요소와 위험

- 일반 사용자 데스크톱 세션에서 트레이, 전역 단축키와 설정 재실행 복원을 직접 확인해야 한다.
- 프로젝트 내부 도구체인은 `.tools`에 있어 시스템 PATH를 수정하지 않는다.

## 다음 작업

1. 일반 사용자 PowerShell에서 `. .\scripts\use-project-rust.ps1`을 실행한다.
2. `npm run tauri -- dev`로 네 창과 트레이 메뉴를 수동 확인한다.
3. 설정 저장 후 재실행 복원과 `Ctrl+Shift+F12` 긴급 중지를 확인한다.
4. 통과 결과를 `docs/13-progress-board.md`에 기록하고 단계 1·2를 완료 처리한다.
5. 작업 3 `펫 이동`으로 넘어간다.

## 완료 게이트

- 트레이에서 펫·설정·타이머 창을 열고 숨길 수 있다.
- 설정을 저장한 뒤 앱을 다시 실행해도 값이 복원된다.
- 손상된 설정 파일이 보관되고 안전한 기본값으로 복구된다.
- 전역 긴급 중지 단축키 등록 성공 또는 사용자에게 충돌 안내가 표시된다.
- TypeScript와 Rust 관련 테스트 및 release build가 통과한다.
