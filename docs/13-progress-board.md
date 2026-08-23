# 13. 진행 현황판

최종 갱신: 2026-08-23
현재 단계: 안전한 발차기 개입 Windows 수동 검증
전체 상태: 실제 전경 창 감지 수동 통과, 유예·재검증·비행 발차기·최소화·쿨다운 자동 검증 완료 — 실제 연출 확인 필요

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
| 9. P1 선택 기능 | 대기 |  |  |  |  |
| 10. 최종 검증 | 대기 |  |  |  |  |

상태 값: `대기`, `진행 중`, `차단`, `완료`, `제외`

## 현재 작업

- 작업: 네모 캐릭터 비행 발차기와 안전한 최소화 실제 환경 검증
- 시작 시각: 2026-08-23
- 목표 종료: 대상 유지 시에만 발차기 후 최소화되고 Alt+Tab·긴급 중지·보호 창에서는 취소되는지 확인 시
- 수정 예정 파일: `docs/17-session-handoff.md`의 다음 작업 참조
- 완료 게이트: 동일 대상 grace 유지, 충돌 직전 fresh 재검증, 최소화 1회, cooldown과 모든 취소 경로 확인

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

## 다음 작업

1. dev 앱을 완전히 재시작하고 테스트 규칙의 유예 시간을 5초로 저장
2. 타이머에서 집중을 시작하고 대상 창을 최대화하지 않은 채 5초 이상 전경 유지
3. 네모 캐릭터가 화면 왼쪽 끝에서 날아와 대상을 차고 작업표시줄로 최소화하는지 확인
4. 다시 대상 창을 열고 발차기 비행 중 Alt+Tab해 새 창과 이전 창 모두 최소화되지 않는지 확인
5. 발차기 비행 중 `Ctrl+Shift+F12`를 눌러 즉시 취소되는지 확인
6. 최대화/전체 화면 창과 설정·작업 관리자에는 개입하지 않는지 확인
7. 동일 창을 다시 열었을 때 cooldown 동안 재개입하지 않는지 확인
8. 통과하면 작업 7을 완료 처리하고 Dragged/Thrown 즉시 취소 연결을 보강

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
현재 상태: 기반 구축 자동 검증 완료, Windows 일반 사용자 세션의 수동 UI 확인 전
마지막 성공 빌드: 2026-08-23 `npm run tauri -- build --no-bundle`
완료한 기능: 창별 TypeScript UI, 트레이, 전역 긴급 중지, 설정 load/save/recovery, 세션 인수인계 규칙
다음으로 할 일: `docs/17-session-handoff.md`의 수동 확인 후 작업 3 펫 이동
알려진 위험: 샌드박스 밖 실제 트레이·단축키·설정 재실행 확인 필요
실행/테스트 방법: `. .\scripts\use-project-rust.ps1` 후 README 명령 실행
```
