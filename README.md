# migam desktop

Windows 11용 데스크톱 펫과 집중 도우미입니다. 감자봇이 작업표시줄을 피해 화면을 돌아다니며 드래그·던지기·상태별 애니메이션, 뽀모도로, 할 일 관리와 안전한 방해 창 개입을 제공합니다.

## 주요 기능

- 투명·항상 위 펫 창, 시스템 트레이와 다중 모니터 작업 영역 이동
- 클릭, 드래그, 던지기와 착지·충돌 애니메이션
- CPU·메모리 사용량에 반응하는 이동과 트레이 표시
- 뽀모도로와 할 일 집중 연결
- 집중 완료 보상 티켓, GAMCHA와 156종 코스튬 인벤토리
- 방해 규칙, 대상 재검증 후 안전한 최소화
- 사진 배달 연출과 `Ctrl+Shift+F12` 긴급 중지
- 설정·할 일·GAMCHA 로컬 저장 및 손상 JSON 보존·복구

방해 창 개입은 기본적으로 꺼져 있으며 사용자가 규칙을 만들고 명시적으로 활성화해야 동작합니다. 실제 창 제목이나 프로세스 경로는 저장하거나 로그로 남기지 않습니다.

## 개발 실행

```powershell
npm install
npm run tauri -- dev
```

## 검증

```powershell
npm run typecheck
npm test
npm run build
cd src-tauri
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings
```

설치 파일은 `npm run tauri -- build`로 만들며 `src-tauri/target/release/bundle/` 아래에 생성됩니다.

## 사용 방법

- 펫 우클릭: 타이머, 할 일, GAMCHA, 설정과 테스트 기능
- 펫 좌클릭 드래그: 위치 이동, 빠르게 놓으면 던지기
- 트레이 메뉴: 창 표시, 긴급 중지, 다시 시작, 종료
- 긴급 중지: `Ctrl+Shift+F12`

## 로컬 데이터와 개인정보

설정·할 일·GAMCHA 진행도는 Tauri 앱 데이터 디렉터리의 `settings.json`, `todo.json`, `gamcha.json`에 저장됩니다. 손상된 파일은 `*.corrupt-<timestamp>.json`으로 보존하고 안전한 기본값으로 복구합니다.

- 계정·클라우드 동기화·분석 기능 없음
- 실제 창 제목과 프로세스 경로 저장·로그 금지
- 관리자·시스템·전체 화면·판정 불가 창은 최소화하지 않음

## 현재 검증 상태

- 프런트엔드 테스트 25개와 Rust 테스트 43개 통과
- TypeScript 검사, Vite production build, rustfmt와 Clippy 통과
- Windows x64 MSI·NSIS release 빌드 성공

실제 DPI, 다중 모니터, 투명 영역 입력, 모든 코스튬 정렬과 장시간 안정성은 Windows 환경별 수동 확인이 필요합니다. 자동 업데이트와 코드 서명은 현재 범위에 포함되지 않습니다.

상세 기준은 [개발 문서 색인](docs/README.md)을 참고하세요.
