# 13. 진행 현황판

최종 갱신: 2026-08-23
현재 단계: GAMCHA 보상 루프 Windows 수동 검증
전체 상태: 집중 자연 완료 보상·영구 저장·156종 룰렛·등급별 연출 자동 검증 완료 — 실제 연출 확인 필요

## 단계별 상태

| 단계 | 상태 | 시작 | 완료 | 담당 | 결과/링크 |
|---|---|---|---|---|---|
| 0. 환경 확인·프로젝트 생성 | 완료 | 2026-08-23 | 2026-08-23 | Codex | 프로젝트 격리 Rust 1.98 MSVC, Node/npm, release build 확인 |
| 1. 앱 셸 | 진행 중 | 2026-08-23 |  | Codex | 다중 창·트레이·전역 단축키 컴파일 완료, 실제 셸 수동 확인 필요 |
| 2. 설정 | 진행 중 | 2026-08-23 |  | Codex | 저장·복구 테스트 통과, UI 재실행 복원 수동 확인 필요 |
| 3. 펫 이동 | 진행 중 | 2026-08-23 |  | Codex | 감자봇 v2 atlas, Idle/Walk, work area clamp와 단위 테스트 완료; Windows 수동 게이트 대기 |
| 4. 클릭·드래그·던지기 | 진행 중 | 2026-08-23 |  | Codex | pointer capture, 속도 판정, 중력·반동·마찰·3초 제한 구현; Windows 수동 게이트 대기 |
| 5. 뽀모도로 | 완료 | 2026-08-23 | 2026-08-23 | Codex | Rust 상태 머신·1초 ticker·Tauri 명령·TypeScript 타이머 UI 및 상태별 버튼 배열 완료 |
| 6. 전경 창과 규칙 | 완료 | 2026-08-23 | 2026-08-23 | Codex | 규칙 UI·저장과 Focus 전용 Win32 일치/불일치 실제 브라우저 검증 완료 |
| 7. 안전한 개입 | 진행 중 | 2026-08-23 |  | Codex | 화면 왼쪽 비행 Kick, grace/cooldown, fresh foreground 재검증과 1회 최소화 구현; Windows 수동 게이트 대기 |
| 8. 긴급 중지 | 대기 |  |  |  |  |
| 9. P1 선택 기능 | 진행 중 | 2026-08-23 |  | Codex | GAMCHA 티켓·룰렛·컬렉션 구현, 코스튬 착용과 Windows 수동 확인 대기 |
| 10. 최종 검증 | 대기 |  |  |  |  |

상태 값: `대기`, `진행 중`, `차단`, `완료`, `제외`

## 현재 작업

- 작업: GAMCHA 집중 완료 보상과 룰렛 실제 환경 검증
- 시작 시각: 2026-08-23
- 목표 종료: 자연 집중 완료 시 티켓이 1장 지급되고 펫 위 룰렛에서 실제 코스튬이 정상 공개되는지 확인 시
- 수정 예정 파일: `docs/17-session-handoff.md`의 다음 작업 참조
- 완료 게이트: Skip/Stop 무보상, 자연 완료 보상, 156종 이미지, 등급 연출, 재시작 후 티켓·컬렉션 복원 확인

## 오늘 완료

- [x] 순수 TypeScript 프런트엔드 방침 확정 및 문서 반영
- [x] 창별 최소 UI와 트레이 코드 작성
- [x] 설정 저장·복구 서비스와 command/UI 작성
- [x] 다음 세션용 `AGENTS.md`, `docs/17-session-handoff.md` 추가
- [x] TypeScript typecheck 및 production build 통과
- [x] 프로젝트 격리 Rust stable MSVC 설치 및 환경 스크립트 추가
- [x] Rust 16개 테스트, rustfmt, Clippy 통과
- [x] 전역 `Ctrl+Shift+F12` 등록과 실패 격리 구현
- [x] Tauri release 앱 빌드 성공
- [x] 기존 감자봇 v2 atlas 구조·투명도·접촉 시트 검증
- [x] 보라색 placeholder를 실제 감자봇 스프라이트 렌더러로 교체
- [x] Idle, 좌·우 Walk 애니메이션과 30fps 이하 창 이동 구현
- [x] monitor work area와 음수 좌표를 지원하는 경계 계산 및 프런트 테스트 4개 추가
- [x] 펫 창과 실제 감자봇 표시 크기를 초기 구현의 50%로 축소
- [x] pointer capture 기반 Dragged 상태와 자동 Walk 중단 구현
- [x] 최근 110ms 포인터 표본 기반 놓기 속도 계산과 700px/s 임계값 구현
- [x] 최대 2,500px/s, 중력 2,200px/s², 반동 0.45, 바닥 마찰 0.80의 Thrown 물리 구현
- [x] Drag/Throw 화면 경계 최소 24px 복구 영역과 최대 3초 종료 구현
- [x] 이동 5개·던지기 5개 프런트 테스트 통과
- [x] Dragged 4프레임, Thrown 6프레임, Landing 4프레임 추가 이미지 생성
- [x] 추가 이미지를 192×208 투명 PNG로 정규화하고 chroma despill·육안 접촉 시트 검수 완료
- [x] 사용자 제공 ‘땅에 박힌 감자봇’ 이미지를 투명 Hard Impact 프레임으로 정리
- [x] Dragged·Thrown·Landing 전용 이미지와 1,400px/s 이상 바닥 충돌 Hard Impact 상태 연결
- [x] 던지기 바닥 경계를 걷기와 같은 완전 표시 최하단으로 통일해 Hard Impact 화면 밖 잘림 수정
- [x] Hard Impact에서 네모 캐릭터·삽을 제거하고 감자봇 단독 박힘 장면으로 교체, 표시 크기·접지점 재조정
- [x] Hard Impact 흙더미의 셀 하단 투명 여백을 0으로 조정해 작업표시줄 바로 위에 밀착
- [x] 기존 Rust PomodoroMachine을 스레드 안전 서비스와 Tauri 명령 6개에 연결
- [x] 앱 내부 1초 ticker로 타이머 창을 숨겨도 Focus/Break 전환 유지
- [x] 타이머 창에 상태·남은 시간·완료 횟수와 시작/일시정지/재개/건너뛰기/중지 UI 구현
- [x] 긴급 중지 시 실행 중 타이머를 일시정지하도록 연결
- [x] 360×280 타이머 창 내부 레이아웃을 재배치하고 문서 스크롤 제거
- [x] 타이머 상태별 사용 가능한 버튼만 노출하도록 제어 배열 정리
- [x] 방해 규칙 이름·활성화·프로세스 파일명·창 제목·유예·재감지 대기 편집 UI 구현
- [x] 규칙 추가·삭제와 개입 명시적 opt-in을 설정 저장에 연결
- [x] 규칙 조건·프로세스 경로 금지·유예 5~600초·재감지 대기 30~3,600초 검증 추가
- [x] `ForegroundWindowSource` 추상화, fake와 Win32 전경 창 snapshot 구현
- [x] Focus 중에만 1초 전경 창 polling하고 실제 제목·경로를 이벤트나 로그에 노출하지 않는 감지 서비스 연결
- [x] 설정 창에 비민감 일치 상태와 규칙 이름 표시
- [x] 사용자 제공 그림에서 네모 캐릭터 단독 투명 발차기 자산 생성·프로젝트 포함
- [x] 화면 왼쪽 바깥에서 대상 창 중앙으로 760ms 비행하는 투명 Kick 창 구현
- [x] 동일 hwnd·rule의 grace 유지와 intervention ID 기반 pending 상태 구현
- [x] 충돌 순간 Focus·긴급 중지·설정·fresh foreground·동일 hwnd·규칙 일치를 재검증한 뒤 `SW_MINIMIZE` 1회 호출
- [x] 대상 변경·Focus 종료·긴급 중지에서 pending Kick 취소
- [x] 앱 자체 PID, 읽을 수 없는 창, 핵심 Windows 프로세스와 전체 화면 창 보호
- [x] 성공/실패 후 동일 hwnd·rule cooldown 적용
- [x] 타이머·설정 창을 WinDbg 분위기의 메뉴·리본·도킹 패널·파란 상태바 UI로 재디자인
- [x] 타이머 360×280 무스크롤 조건과 상태별 버튼 배열 유지
- [x] `pack/manifest.json`의 기본 3종을 제외한 156종 코스튬을 GAMCHA 후보로 연결
- [x] 자연 집중 완료에만 티켓 1장을 지급하고 Skip/Stop 보상을 차단
- [x] 일반 60%·레어 25%·에픽 10%·전설 4%·스페셜 1% 확률과 등급 내 중복 방지 구현
- [x] 티켓·누적 뽑기·컬렉션을 앱 데이터 `gamcha.json`에 원자적으로 저장·복원
- [x] 펫 위 GAMCHA 말풍선, 무지개 로고, 룰렛 감속, 등급별 빛·회전·공개 연출 구현
- [x] 펫 우클릭과 트레이에 `GAMCHA!` 열기 메뉴 추가
- [x] 타이머 UI polling이 먼저 단계 전환할 때 티켓이 누락되던 경쟁 조건 수정 및 추첨 전 빈 이미지 숨김
- [x] 집중 완료 시 전체 GAMCHA 창 대신 펫 위 클릭형 보상 말풍선만 표시하도록 2단계 진입 구조로 변경
- [x] 말풍선 클릭 후 펫 모니터 전체화면에서 32개 아이템 이중 궤도·회전 광선·56개 색종이·24프레임 셔플 연출 구현
- [x] 불투명 전체화면 배경을 제거하고 기존 화면 위 투명 오버레이로 변경, 아이템을 모니터 네 변의 직사각형 경로로 양방향 회전
- [x] 타이머 초기 IPC 실패 시 긴 오류 문장이 작은 창에 잘리던 문제를 자동 재연결 가능한 소형 상태로 교체
- [x] GAMCHA 중앙 결과의 사각 방사 배경·이중 테두리를 제거하고 무경계 원형 광원으로 변경, 뽑기 버튼을 소형 단색+무지개 선으로 정리
- [x] GAMCHA 보유 코스튬 옷장, 착용·해제, 영구 저장과 펫 실시간 오버레이 구현
- [x] 256×256 코스튬을 128×128로 축소하고 96×104 펫 셀 중심에 `left -16px`, `top -12px` 공통 정렬

## 다음 작업

1. dev 앱을 완전히 재시작하고 집중 시간을 1분으로 저장
2. 집중을 시작해 건너뛰지 않고 1분을 끝낸 뒤 GAMCHA 말풍선과 티켓 1장을 확인
3. 룰렛을 눌러 감속 애니메이션, 실제 코스튬 이름·이미지와 등급별 연출 확인
4. 앱을 재시작해 티켓·컬렉션 수가 유지되는지 확인
5. Skip과 Stop으로는 티켓이 지급되지 않는지 확인
6. 수동 결과에 따라 말풍선 크기·위치·연출 강도를 조정
7. 다음 구현에서 당첨 코스튬을 실제 펫에 정렬·착용하는 기능 추가

## 차단 요소

| ID | 문제 | 영향 | 대응 | 상태 |
|---|---|---|---|---|
| B-001 | Rust/Cargo를 현재 환경에서 찾을 수 없음 | Rust 테스트·Tauri 실행 불가 | 프로젝트 내부 Rust stable MSVC 설치 | 닫힘 |
| B-002 | Codex 샌드박스가 Windows GUI 창 생성을 거부 | 자동 실행 smoke test 불가 | 일반 사용자 PowerShell에서 수동 실행 | 열림 |
| B-003 | 샌드박스 네트워크가 WiX 다운로드를 차단 | MSI 번들 생성 불가 | `tauri build --no-bundle`로 release exe 검증; 설치파일은 네트워크 가능한 환경에서 생성 | 열림 |

## 발견된 버그

| ID | 심각도 | 현상 | 재현 | 담당 | 상태 |
|---|---|---|---|---|---|
| BUG-001 |  |  |  |  |  |
| BUG-002 | S1 | Windows에서 Vite가 잠긴 Rust `.exe`를 감시해 EBUSY 종료 | `npm run tauri -- dev` | Codex | 수정 완료 — `src-tauri/target`, `.tools` 감시 제외 |

심각도:

- S0: 데이터/시스템 안전 문제, 즉시 중단
- S1: 핵심 경로 실패
- S2: 우회 가능한 기능 문제
- S3: 시각/문구 문제

## 테스트 기록

| 시각 | 명령/검사 | 결과 | 메모 |
|---|---|---|---|
| 2026-08-23 | `npm run typecheck` | 통과 | TypeScript 오류 없음 |
| 2026-08-23 | `npm run build` | 통과 | Vite production build 성공 |
| 2026-08-23 | `npm test` | 통과 | 프런트 테스트 파일은 아직 없음 |
| 2026-08-23 | Rust/Cargo 확인 | 차단 | 실행 파일을 찾지 못함 |
| 2026-08-23 | 감자봇 atlas deterministic validation | 통과 | 1536×2288, 8×11, 오류·경고 없음, despill `ok: true` |
| 2026-08-23 | `npm test` | 통과 | 펫 work area·clamp·target·이동 순수 함수 4개 |
| 2026-08-23 | `npm run typecheck` | 통과 | 감자봇 렌더러와 Tauri 이동 adapter 포함 |
| 2026-08-23 | `cargo test` | 통과 | Rust 16개 테스트 |
| 2026-08-23 | `cargo clippy --all-targets -- -D warnings` | 통과 | 오류 없음 |
| 2026-08-23 | `npm run tauri -- build --no-bundle` | 통과 | release exe 생성, 감자봇 WebP 번들 포함 |
| 2026-08-23 | `npm run tauri -- build` | 부분 통과 | release exe 생성 후 WiX 네트워크 다운로드만 차단 |
| 2026-08-23 | 50% 크기 변경 후 `npm test`, `npm run build` | 통과 | 128×128 창, 96×104 스프라이트 production build 확인 |
| 2026-08-23 | `tauri build --debug --no-bundle` | 실행 파일 잠금 | 실행 중인 dev 앱이 `target/debug/desktop-pet-mvp.exe`를 사용 중; 앱 종료 후 재검증 가능 |
| 2026-08-23 | Drag/Throw 구현 후 `npm test` | 통과 | 이동 5개, 포인터 속도·중력·반동·3초 제한 5개, 총 10개 |
| 2026-08-23 | Drag/Throw 구현 후 `npm run typecheck`, `npm run build` | 통과 | TypeScript와 production asset 빌드 성공 |
| 2026-08-23 | Drag/Throw 구현 후 `tauri build --no-bundle` | 통과 | release exe 생성 |
| 2026-08-23 | Rust toolchain 설치 | 통과 | 프로젝트 `.tools`에 rustc 1.98.0 MSVC 설치 |
| 2026-08-23 | `cargo test` | 통과 | 16개 테스트 통과 |
| 2026-08-23 | `cargo fmt --check`, `cargo clippy -D warnings` | 통과 | linker 현지화 메시지는 허용 |
| 2026-08-23 | `npm run tauri -- build --no-bundle` | 통과 | release 실행 파일 생성 |
| 2026-08-23 | release 실행 smoke test | 차단 | 샌드박스가 Tauri GUI 창 생성 시 access denied 반환 |
| 2026-08-23 | Vite watcher 설정 | 수정 | Rust 빌드 결과와 프로젝트 도구 폴더 감시 제외 |
| 2026-08-23 | 추가 동작 이미지 deterministic QA | 통과 | Dragged 4장, Thrown 6장, Landing 4장 모두 192×208 RGBA, 투명 배경, despill 완료 |
| 2026-08-23 | Hard Impact 연결 후 `npm test`, `npm run typecheck`, `npm run build` | 통과 | 총 11개 테스트, 추가 PNG 15장 production asset 포함 |
| 2026-08-23 | 바닥 경계 정렬 수정 후 `npm test`, `npm run typecheck`, `npm run build` | 통과 | 걷기·바닥 반동·Hard Impact가 동일한 완전 표시 바닥선 사용 |
| 2026-08-23 | Hard Impact 단독 캐릭터 자산 QA | 통과 | 감자봇과 연결된 흙더미만 표시, 192×208 RGBA, 실제 투명 배경, 캐릭터 크기 정렬 |
| 2026-08-23 | Hard Impact 하단 접지 QA | 통과 | 불투명 영역 bbox 하단 208px로 셀 최하단에 정확히 밀착 |
| 2026-08-23 | Pomodoro 구현 후 `cargo test` | 통과 | Rust 20개 테스트, 서비스 상태·설정 반영 포함 |
| 2026-08-23 | Pomodoro 구현 후 `npm test` | 통과 | 프런트 16개 테스트, 타이머 표시·제어 상태 포함 |
| 2026-08-23 | Pomodoro 구현 후 typecheck/build/Clippy | 통과 | TypeScript production build와 Rust `-D warnings` 통과 |
| 2026-08-23 | Pomodoro 구현 후 `tauri build --no-bundle` | 통과 | release 실행 파일 생성 |
| 2026-08-23 | 타이머 창 무스크롤 레이아웃 | 수정 | 축소 여백, 단일 제어 행, timer 문서 overflow 차단 |
| 2026-08-23 | 타이머 버튼 배열 개선 | 수정 | 대기 1개, 실행 중 3개, 일시정지 2개만 균등 표시 |
| 2026-08-23 | 전경 창·규칙 구현 후 `cargo test` | 통과 | Rust 24개 테스트, Focus 외 source 미호출과 비민감 일치 결과 포함 |
| 2026-08-23 | 전경 창·규칙 구현 후 `npm test`, `npm run build` | 통과 | 프런트 16개 테스트와 TypeScript production build 통과 |
| 2026-08-23 | 전경 창·규칙 구현 후 fmt/Clippy | 통과 | Rust format과 `-D warnings` 통과 |
| 2026-08-23 | Kick 투명 자산 QA | 통과 | 1536×1024 RGBA, corner alpha 0, 네모 캐릭터 단독 발차기 |
| 2026-08-23 | 안전 개입 구현 후 `cargo test` | 통과 | Rust 24개 테스트, grace·fresh 재검증·긴급 중지 취소 포함 |
| 2026-08-23 | 안전 개입 구현 후 `npm test`, `npm run build` | 통과 | 프런트 16개 테스트, Kick PNG production asset 포함 |
| 2026-08-23 | 안전 개입 구현 후 Clippy | 통과 | `cargo clippy --all-targets -- -D warnings` 통과 |
| 2026-08-23 | WinDbg UI 적용 후 `npm test`, `npm run build` | 통과 | 프런트 16개 테스트와 production build 통과 |
| 2026-08-23 | WinDbg raw UI 조정 후 `npm test`, `npm run build` | 통과 | 명령줄·고전 Win32 경계·회색 버튼·고밀도 패널 적용, 프런트 16개 테스트 통과 |
| 2026-08-23 | 펫 우클릭 메뉴·타이머 말풍선 연결 후 전체 검사 | 통과 | 네이티브 우클릭 메뉴, 펫 추적 말풍선 위치, 프런트 16개·Rust 24개·Clippy·production build 통과 |
| 2026-08-23 | 집중 모드·초소형 타이머 적용 후 `npm test`, `npm run build` | 통과 | 집중 중 이동 정지, 컴퓨터 작업 RGBA 자산, 156×76 표시 전용 말풍선 연결 |
| 2026-08-23 | Chrome 감지·Kick 포커스 수정 후 Rust 검사 | 통과 | ToolHelp 프로세스명 fallback, 비포커스 Kick 창, 250ms 감지 주기 적용 |
| 2026-08-23 | GAMCHA 구현 후 `npm test`, `npm run build` | 통과 | 프런트 18개 테스트, 156종 후보 production asset 포함 |
| 2026-08-23 | GAMCHA 구현 후 Rust 검사 | 통과 | Rust 32개 테스트, fmt, Clippy `-D warnings` 통과 |
| 2026-08-23 | GAMCHA 구현 후 `tauri build --no-bundle` | 통과 | release 실행 파일 생성 |
| 2026-08-23 | GAMCHA 티켓 경쟁 조건 수정 후 전체 검사 | 통과 | 모든 Tick 경로에서 동일 보상 처리, 프런트 18개·Rust 32개·Clippy·build 통과 |
| 2026-08-23 | GAMCHA 2단계 전체화면 연출 후 전체 검사 | 통과 | `gamcha-notice` 창, 전체화면 전환, 프런트 18개·Rust 32개·Clippy·release build 통과 |
| 2026-08-23 | GAMCHA 투명 사각 궤도 수정 후 전체 검사 | 통과 | 기존 화면 유지, 중앙 결과 카드, 프런트 18개·Rust 32개·Clippy·build 통과 |
| 2026-08-23 | 코스튬 착용 구현 후 전체 검사 | 통과 | 옷장·해제·저장·보유 검증·펫 오버레이, 프런트 18개·Rust 33개·Clippy·release build 통과 |
| 2026-08-23 | 코스튬 정렬·집중 감시 복구 수정 | 통과 | 슬롯별 착용 기준점, Tick 일시 오류 후 감시 지속, 프런트 18개·Rust 33개·임시 경로 production build 통과 |
| 2026-08-23 | 코스튬별 수동 보정 기반 구현 | 통과 | 옷장 X·Y·크기 조절/초기화, 코스튬별 영구 저장, 프런트 20개·Rust 34개·Clippy·production build 통과 |
| 2026-08-23 | CPU·메모리 반응형 펫 구현 | 통과 | CPU/메모리/통합/끔 선택, 실시간 사용률, 속도·대기·바쁨·실패 동작 연결, 프런트 23개·Rust 36개·Clippy·production build 통과 |
| 2026-08-23 | WinDbg 펫 우클릭 팝업 구현 | 통과 | 네이티브 메뉴를 상태 패널·명령 버튼·화면 경계 보정 전용 창으로 교체, 프런트 23개·Rust 36개·production build 통과 |
| 2026-08-23 | CPU·MEM 감자봇 트레이 표시 구현 | 통과 | CPU 파랑·MEM 빨강 2개 아이콘, 10단계 게이지·4단계 표정·실시간 툴팁, 프런트 23개·Rust 38개·Clippy·production build 통과 |
| 2026-08-23 | CPU·MEM 트레이 가독성 개선 | 통과 | 방사형 게이지를 전폭 하단 막대로 교체하고 얼굴을 약 40% 확대, Rust 38개·Clippy 통과 |
| 2026-08-23 | 부하 구간별 달리기 연결 | 통과 | 0~19 기본 걷기, 20~39 alert, 40~59 medium, 60~79 fast, 80~100 extreme 각 4프레임·좌우 방향·단계별 속도 연결, 프런트 24개·typecheck·production build 통과 |
| 2026-08-23 | 시스템 반응 연속 달리기 | 통과 | CPU/MEM/통합 모드에서는 목적지 도착 즉시 다음 목적지를 선택해 대기 없이 계속 이동, 사용 안 함에서만 기존 휴식 유지, 프런트 25개·typecheck·production build 통과 |
| 2026-08-23 | 시스템 반응 기준 UI 정렬 | 수정 | 둥근 기본 select를 크기 입력과 동일한 각진 WinDbg 테두리·높이·Consolas 글꼴·포커스 표시로 통일 |

## 시간 예산

| 구분 | 계획 | 실제 | 차이 |
|---|---:|---:|---:|
| 환경·scaffold·앱 셸 | 4.5h |  |  |
| 설정 | 1.5h |  |  |
| 펫 이동·상호작용 | 3.5h |  |  |
| 타이머·규칙 UI | 3h |  |  |
| 감지·개입·긴급 중지 | 6h |  |  |
| 테스트·릴리스 | 5.5h |  |  |

## 마지막 인수인계

```text
현재 상태: CPU·MEM 감자봇 트레이 아이콘 2개 구현 완료, Windows 수동 확인 전
마지막 성공 검사: 2026-08-23 프런트 23개·Rust 38개·typecheck·fmt·Clippy·별도 임시 경로 production build 통과
완료한 기능: CPU 파랑·MEM 빨강 트레이 표시, 전폭 사용률 막대, 부하 5구간별 걷기/달리기 16프레임, 좌우 방향, 단계별 이동·재생 속도
다음으로 할 일: 앱 재시작 후 CPU/MEM/통합 기준에서 목적지 사이에 멈춤 없이 계속 달리는지와 각 구간 이미지 크기·바닥선·좌우 반전을 확인
알려진 위험: Windows 설정에 따라 새 트레이 아이콘이 `숨겨진 아이콘 표시(^)` 안에 처음 배치될 수 있음
실행/테스트 방법: `. .\scripts\use-project-rust.ps1` 후 README 명령 실행
```
