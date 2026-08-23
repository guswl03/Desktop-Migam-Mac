# 세션 인수인계

최종 갱신: 2026-08-23

이 문서는 다음 작업 세션이 가장 먼저 확인하는 단일 인수인계 기록이다. 작업을 끝낼 때마다 오래된 내용을 남기지 말고 현재 상태로 갱신한다.

## 현재 목표

작업 9 `GAMCHA`의 집중 완료 보상·룰렛 Windows 수동 게이트를 통과하고 당첨 코스튬 착용 정렬을 구현한다.

## 현재 상태

- Tauri 2 + Rust + Vite + 순수 TypeScript 기반 구축이 완료되어 있다.
- `pet`, `card`, `timer`, `settings`, `gamcha` 창과 트레이, 설정 저장·복구, 전역 긴급 중지가 구현되어 있다.
- 집중 시간이 자연스럽게 끝날 때만 GAMCHA 티켓 1장이 지급된다. Skip과 Stop은 보상하지 않는다.
- GAMCHA 티켓·누적 추첨·보유 코스튬은 앱 데이터 디렉터리의 `gamcha.json`에 저장되고 손상 파일은 별도로 보존한다.
- `pack/manifest.json` 159종 중 `default` 3종을 제외한 156종이 실제 추첨 후보이다.
- 확률은 Common 60%, Rare 25%, Epic 10%, Legendary 4%, Special 1%이다.
- 선택된 등급 안에서는 그 등급의 모든 코스튬을 모을 때까지 중복이 나오지 않는다.
- 자연 집중 완료 시에는 작은 `gamcha-notice` 보상 말풍선만 펫 위에 나타난다.
- 보상 말풍선을 클릭하면 펫이 위치한 모니터 크기의 투명 `gamcha` 오버레이가 열리며, 기존 바탕 화면과 앱 창은 그대로 보인다.
- 추첨은 32개 실제 코스튬이 모니터의 위·오른쪽·아래·왼쪽 네 변을 따라 약 0.9초마다 양방향 회전하고, 결과 카드는 화면 정중앙에 표시된다. 56개 색종이, 중앙 24프레임 감속 셔플과 등급별 공개 효과도 유지한다.
- 중앙 결과는 사각 프레임 없이 경계가 사라지는 원형 등급 광원 위에 아이템·이름만 표시한다. 뽑기 버튼은 작은 반투명 단색 버튼과 상단 3px 무지개 선을 사용한다.
- 타이머 UI polling과 백그라운드 ticker 중 어느 쪽이 먼저 자연 완료를 처리해도 공통 Tick 경로에서 티켓을 지급한다.
- GAMCHA 왼쪽 아래 `OUTFIT` 옷장에서 보유 코스튬 또는 `기본 모습`을 선택해 착용·해제할 수 있다.
- 착용 ID는 `gamcha.json`에 저장되고 `gamcha://equipped` 이벤트로 펫에 즉시 반영되며 재시작 후 복원된다. 보유하지 않은 ID는 Rust에서 거부한다.
- 256×256 코스튬은 128×128로 표시하고 96×104 펫 셀에 `left -16px`, `top -12px`로 중심 정렬한다. 걷기 중 2px 바운스를 적용하고 Hard Impact에서는 숨긴다.
- `pack`의 개별 자산별 runtime alignment 메타데이터는 아직 없으므로 모자·안경·몸 장식별 Windows 육안 확인 후 종류별 미세 보정이 다음 단계다.
- 보라색 placeholder 대신 `images/characters/gamjabot/final/spritesheet-extended.webp`의 실제 감자봇을 표시한다.
- 감자봇 atlas는 1536×2288, 192×208 셀, 8열×11행이며 deterministic validation과 chroma despill이 통과했다.
- 펫 창은 128×128이며, HTML/root/body/app/shell 배경을 모두 투명하게 두어 감자봇 외 픽셀을 그리지 않는다.
- 원본 192×208 셀을 96×104로 표시해 초기 구현보다 감자봇 크기를 정확히 절반으로 줄였다.
- Idle 6프레임, running-right 8프레임, running-left 8프레임을 실제 atlas에서 재생한다.
- 펫은 1.8~4.6초 Idle 후 현재 모니터 작업 영역의 바닥선을 따라 좌우 Walk한다.
- 현재 모니터의 physical pixel work area를 사용하며 작업표시줄을 제외하고, 일반 창은 완전히 안쪽에 유지한다.
- 창이 작업 영역보다 큰 예외 상황에는 최소 24px가 보이도록 경계를 계산한다.
- 창 이동 권한은 `pet` 창에만 별도 capability로 허용한다.
- 감자봇을 왼쪽 버튼으로 누르면 Dragged가 자동 Idle/Walk를 즉시 중단한다.
- 드래그는 Tauri physical cursor 좌표와 pointer capture를 사용하므로 DPI와 창 이동 중에도 같은 잡기 지점을 유지한다.
- 최근 110ms 포인터 표본이 700px/s 이상이면 Thrown으로 전환하고 최대 속력은 2,500px/s로 제한한다.
- Thrown은 중력 2,200px/s², 반동 계수 0.45, 바닥 마찰 0.80을 사용하고 최대 3초 안에 종료한다.
- 드래그와 던지기 중에도 현재 모니터 work area 기준 최소 24px가 화면에 남는다.
- 던지는 동안 atlas의 jumping 5프레임을 재생한다.
- `images/characters/gamjabot/extra/frames`에 Dragged 4장, Thrown 6장, Landing 4장의 전용 투명 PNG가 준비되어 있다.
- 추가 프레임은 모두 192×208 RGBA이며 `extra/manifest.json`에 재생 순서와 권장 프레임 시간이 기록되어 있다.
- `extra/qa/contact-sheet-normalized.png`와 상태별 GIF로 투명 배경, 캐릭터 단독 표시, chroma despill을 육안 검수했다.
- 사용자 제공 ‘땅에 박힌 감자봇’ 그림은 네모 캐릭터와 삽을 제거해 감자봇과 연결된 흙더미만 남겼으며, `extra/frames/hard-impact/00.png`의 192×208 투명 Hard Impact 프레임으로 정리했다.
- Hard Impact의 감자봇 머리 크기는 일반 착지 프레임과 비슷하게 맞추고 흙더미 바닥선을 셀 하단에 정렬했다.
- Hard Impact 프레임의 흙더미 불투명 픽셀은 208px 셀 최하단까지 닿아 작업표시줄과 투명 틈이 생기지 않는다.
- 드래그 중에는 Dragged 4프레임, 비행 중에는 Thrown 6프레임, 일반 종료에는 Landing 4프레임을 재생한다.
- 바닥 충돌 직전 하강 속도가 1,400px/s 이상이면 반동을 멈추고 Hard Impact를 1초 표시한 뒤 Idle로 돌아간다.
- 걷기·던지기 바닥 반동·Hard Impact는 모두 창 전체가 보이는 동일한 작업 영역 최하단을 사용한다. 드래그 중에만 복구를 위해 최소 24px 표시 경계를 허용한다.
- Rust `PomodoroMachine`은 `PomodoroService`를 통해 앱 전역 상태로 연결되어 있다.
- 타이머는 Stopped, Focus, ShortBreak, LongBreak, Paused 상태와 절대 종료 시각 기반 남은 시간을 사용한다.
- 앱 내부 1초 ticker가 타이머 창 표시 여부와 무관하게 단계 완료를 처리한다.
- `get_timer_state`, `start_focus`, `pause_timer`, `resume_timer`, `skip_phase`, `stop_timer` Tauri 명령이 연결되어 있다.
- 타이머 창은 상태 문구, MM:SS 남은 시간, 완료한 집중 횟수와 모든 제어 버튼을 표시한다.
- 타이머 창은 360×280 안에 맞고 문서 스크롤이 없다. 대기에는 시작만, 실행 중에는 일시정지·건너뛰기·중지, Paused에는 재개·중지만 균등 표시한다.
- 설정 변경은 Stopped 타이머에 즉시 적용되며 실행 중 변경은 다음 Stop 이후부터 적용된다.
- 전역 긴급 중지는 실행 중인 타이머를 Paused로 전환한다.
- 설정 창에서 방해 규칙 이름, 사용 여부, 프로세스 파일명, 창 제목 포함 문자열, 유예 시간과 재감지 대기를 추가·삭제·편집할 수 있다.
- 프로세스명/창 제목 중 하나 이상이 필요하고 프로세스 경로 입력은 거부한다. 유예는 5~600초, 재감지 대기는 30~3,600초다.
- 방해 감지는 기본 off이며 활성 규칙이 있어야 사용자가 명시적으로 켤 수 있다.
- `ForegroundWindowSource` trait과 fake, Win32 `GetForegroundWindow`/PID/프로세스 파일명/제목 snapshot 구현이 있다.
- 전경 창 source는 Focus 실행 중이면서 감지가 켜진 경우에만 1초마다 호출된다.
- 감지 결과 이벤트에는 실제 창 제목·프로세스 경로를 넣지 않고 일치 여부와 사용자가 만든 규칙 ID만 넣는다.
- 설정 창은 감지 결과를 `일치 감지됨 · 규칙 이름` 또는 `일치하는 전경 창 없음`으로 표시한다.
- 사용자가 실제 Chrome/YouTube에서 규칙 일치 표시를 확인해 작업 6 감지 게이트를 통과했다.
- 사용자 제공 그림에서 네모 캐릭터만 분리·복원한 투명 Kick 자산을 `extra/frames/kick/00.png`에 추가했다.
- Kick 전용 투명 `card` 창은 220×180이며 대상 모니터의 왼쪽 화면 밖에서 대상 창 중앙까지 760ms 동안 날아온다.
- 동일 hwnd와 rule이 설정된 grace 동안 유지되어야 Kick이 시작된다.
- 충돌 순간 Focus, 긴급 중지, 개입 설정, fresh foreground hwnd와 규칙 일치를 다시 확인하고 `ShowWindow(SW_MINIMIZE)`를 한 번만 호출한다.
- 대상 변경, Focus 종료와 긴급 중지는 pending Kick을 취소한다.
- 앱 자체 PID, 읽을 수 없는 창, 작업 관리자·Explorer 등 핵심 프로세스와 전체 화면 창은 보호한다.
- 개입 성공/실패 뒤 동일 hwnd·rule에 설정된 cooldown을 적용한다.
- 타이머와 설정 창은 WinDbg를 참고한 밝은 회색 도구 UI다. 파란 활성 탭·상태바, 리본형 상단 영역, 얇은 도킹 패널 경계와 Consolas 상태 표시를 사용한다.
- 타이머의 360×280 무스크롤 조건과 상태별 실제 제어 버튼 수는 WinDbg 스타일 변경 뒤에도 유지된다.

## 이번 세션 변경 파일

- `src/gamcha/gamcha-model.ts`
- `src/gamcha/gamcha-model.test.ts`
- `src/gamcha/gamcha-view.ts`
- `src-tauri/src/domain/gamcha.rs`
- `src-tauri/src/application/gamcha_service.rs`
- `src-tauri/src/presentation/tray.rs`
- `src-tauri/capabilities/default.json`

- `src/contracts.ts`
- `src/main.ts`
- `src/intervention/kick-view.ts`
- `src/styles.css`
- `src/pet/motion.ts`
- `src/pet/motion.test.ts`
- `src/pet/physics.ts`
- `src/pet/physics.test.ts`
- `src/pet/sprite.ts`
- `src/pet/tauri-motion-runtime.ts`
- `src/timer/timer-view.ts`
- `src/timer/timer-view.test.ts`
- `src-tauri/src/application/pomodoro_service.rs`
- `src-tauri/src/application/foreground_monitor.rs`
- `src-tauri/src/domain/pomodoro.rs`
- `src-tauri/src/domain/distraction.rs`
- `src-tauri/src/domain/foreground.rs`
- `src-tauri/src/domain/settings.rs`
- `src-tauri/src/infrastructure/mod.rs`
- `src-tauri/src/infrastructure/windows/mod.rs`
- `src-tauri/src/infrastructure/windows/foreground_window.rs`
- `src-tauri/src/app_state.rs`
- `src-tauri/src/presentation/commands.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/capabilities/pet-motion.json`
- `src-tauri/tauri.conf.json`
- `docs/13-progress-board.md`
- `docs/17-session-handoff.md`
- `images/characters/gamjabot/extra/manifest.json`
- `images/characters/gamjabot/extra/frames/dragged/*.png`
- `images/characters/gamjabot/extra/frames/thrown/*.png`
- `images/characters/gamjabot/extra/frames/landing/*.png`
- `images/characters/gamjabot/extra/frames/hard-impact/00.png`
- `images/characters/gamjabot/extra/frames/kick/00.png`
- `images/characters/gamjabot/extra/qa/contact-sheet-normalized.png`
- `images/characters/gamjabot/extra/qa/previews/*.gif`

## 검증 상태

- GAMCHA 구현 후 `npm test`: 프런트 18개 테스트 통과.
- GAMCHA 구현 후 `cargo test`: Rust 32개 테스트 통과.
- `cargo fmt`, `cargo clippy --all-targets -- -D warnings`: 통과.
- `npm run tauri -- build --no-bundle`: 통과, release 실행 파일 생성.
- 일반 사용자 Windows에서 자연 완료 보상, 말풍선 위치, 등급별 연출과 재시작 복원은 수동 확인이 필요하다.
- 사용자 확인에서 발견된 `휴식 전환 후 TICKET 0` 경쟁 조건과 추첨 전 깨진 빈 이미지는 수정했고 전체 검사를 재통과했다.
- 타이머 초기 연결 실패 시 `타이머를 불러오지 못했습니다`가 작은 말풍선에서 크게 잘리던 UI를 제거했다. 이제 `재연결 --:--` 상태에서 500ms polling으로 자동 복구한다.
- GAMCHA 2단계 전체화면 변경 후 프런트 18개·Rust 32개·Clippy와 release build를 재통과했다.
- 코스튬 착용 구현 후 프런트 18개·Rust 33개, Clippy와 release build를 통과했다.
- 코스튬을 이름 기준 `head`·`face`·`neck`·`body`·`full` 슬롯으로 분류하고 슬롯별 크기와 기준점을 적용했다. 사용자에게 보인 `rare_025` 연금술사 보석모는 머리 위에 작게 정렬된다.
- 백그라운드 Tick이 일시적으로 실패해도 스레드를 종료하지 않고 다음 주기에 복구한다. Tick 상태 이벤트 전달도 best-effort로 바꿔 집중 중 전경 감시와 Kick이 함께 영구 중단되는 경로를 제거했다.
- 위 수정 후 `npm test` 프런트 18개, TypeScript 검사, 별도 임시 경로 Vite production build, `cargo fmt --check`, Rust 33개 테스트를 통과했다. 실행 중 앱이 기본 `dist`와 `target`을 잠근 상태라 검증 출력만 임시 경로를 사용했다.
- GAMCHA 옷장에 코스튬별 X(-80~80), Y(-80~80), 크기(48~180) 슬라이더와 `위치 초기화`를 추가했다. 선택한 코스튬을 먼저 적용한 뒤 조절하면 펫에 약 120ms 단위로 반영되고 `gamcha.json`에 코스튬 ID별로 저장된다.
- 저장되지 않은 코스튬은 기존 슬롯 기본값을 사용하며, 과거 `gamcha.json`은 새 `costumeAlignments` 필드가 없어도 자동으로 빈 값으로 읽는다.
- 개별 보정 기반 구현 후 프런트 20개, Rust 34개 테스트, TypeScript, Vite production build와 Clippy `-D warnings`를 통과했다.

- 감자봇 `final/validation-extended.json`: `ok: true`, 1536×2288, 8×11, 오류·경고 없음.
- 감자봇 `qa/chroma-despill-extended.json`: `ok: true`.
- `npm test`: 펫 경계·음수 모니터 좌표·최소 24px·목적지·이동 테스트 4개 통과.
- `npm run typecheck`: 통과.
- `npm run build`: 통과. 감자봇 WebP가 production asset에 포함됨.
- 50% 크기 변경 후 `npm test`와 `npm run build` 재통과.
- Drag/Throw 구현 후 `npm test`: 이동 5개와 던지기 물리 5개, 총 10개 통과.
- Drag/Throw 구현 후 `npm run typecheck`, `npm run build`: 통과.
- Drag/Throw 구현 후 `npm run tauri -- build --no-bundle`: 통과, release exe 생성.
- `cargo test`: 기존 Rust 테스트 16개 통과.
- `cargo clippy --all-targets -- -D warnings`: 통과.
- `npm run tauri -- build --no-bundle`: 통과, release exe 생성.
- 전체 `tauri build`는 release exe 생성 후 WiX 다운로드가 샌드박스 네트워크 정책으로 차단되어 MSI만 만들지 못했다.
- Codex 샌드박스에서는 Windows GUI 동작을 직접 볼 수 없어 투명 창과 실제 이동은 일반 사용자 세션 확인이 필요하다.
- 크기 변경 뒤 debug Tauri build는 실행 중인 기존 dev 앱이 `target/debug/desktop-pet-mvp.exe`를 잠가 교체하지 못했다. 앱 종료 후 다시 실행하면 된다.
- 추가 동작 이미지 14장은 192×208 RGBA로 정규화되었고, 프레임 검사에서 오류·경고가 없으며 최종 chroma despill을 통과했다.
- Hard Impact 연결 후 `npm test` 11개, `npm run typecheck`, `npm run build`가 모두 통과했다.
- 바닥 경계 정렬 수정 후 자동 테스트는 12개이며 Hard Impact가 화면 아래로 잘리던 원인을 제거했다.
- Pomodoro 구현 후 `cargo test`: Rust 20개 테스트 통과.
- Pomodoro 구현 후 `npm test`: 프런트 16개 테스트 통과.
- Pomodoro 구현 후 `npm run build`, `cargo fmt`, `cargo clippy --all-targets -- -D warnings`: 통과.
- Pomodoro 구현 후 `npm run tauri -- build --no-bundle`: 통과, release 실행 파일 생성.
- 전경 창·규칙 구현 후 `cargo test`: Rust 24개 테스트 통과.
- 전경 창·규칙 구현 후 `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`: 통과.
- 전경 창·규칙 구현 후 `npm test`: 프런트 16개 테스트 통과, `npm run build`: 통과.
- 안전 개입 구현 후 `cargo test`: Rust 24개 테스트 통과, grace 후 시작·fresh 재검증·긴급 중지 취소 포함.
- 안전 개입 구현 후 `cargo clippy --all-targets -- -D warnings`: 통과.
- 안전 개입 구현 후 `npm test`: 프런트 16개 테스트 통과, `npm run build`: Kick PNG 포함 production build 통과.
- WinDbg UI 적용 후 `npm test`: 프런트 16개 테스트 통과, `npm run build`: 통과.
- WinDbg raw UI 조정 후 `npm test`: 프런트 16개 테스트 통과, `npm run build`: 통과. 타이머와 설정창을 고전 Win32 디버거의 조밀하고 거친 형태로 조정했다.
- 펫 우클릭에 Windows 네이티브 메뉴를 연결해 타이머·설정·긴급 중지·재시작·종료를 실행할 수 있다.
- 타이머 창은 156×76 투명 무장식 말풍선이며 `상태 · 남은 시간`만 표시하고, 열린 동안 250ms 간격으로 펫 위치를 따라간다. 제어는 펫 우클릭·트레이 메뉴로 이동했다.
- 우클릭 메뉴·말풍선 연결 후 `npm test` 16개, `cargo test` 24개, `cargo clippy --all-targets -- -D warnings`, `npm run build`가 통과했다.
- 집중 타이머가 활성화되면 펫의 자동 이동과 드래그가 중단된다. Focus에서는 컴퓨터 작업 전용 이미지 `images/characters/gamjabot/extra/frames/focused/00.png`를 표시하고, 휴식·일시정지에도 제자리에서 대기한다.
- 집중 전용 이미지는 built-in imagegen으로 생성·배경 추출했으며 1186×1327 RGBA, corner alpha 0을 확인했다.
- Chrome 프로세스 경로 조회가 실패하면 ToolHelp 프로세스 목록에서 파일명을 재확인한다. Kick 창은 `focusable: false`로 대상 창의 전경 상태를 빼앗지 않으며 감지 주기는 250ms다.

## 차단 요소와 위험

- 일반 사용자 데스크톱에서 감자봇 외 배경이 완전히 투명한지 확인해야 한다.
- 단일 모니터에서 작업표시줄을 침범하지 않는지, 가능한 경우 보조 모니터/다른 DPI에서도 확인해야 한다.
- 설정의 `visualScalePercent`는 아직 실제 펫 창/스프라이트 크기에 적용되지 않는다.
- v2 자산의 16방향 blind visual QA는 원래 생성 환경의 ACL 문제로 완료되지 않았지만, 이번 단계에서 사용하는 행 0~2는 deterministic validation과 접촉 시트 육안 확인을 마쳤다.
- pointer capture가 움직이는 투명 Tauri 창에서도 release까지 유지되는지 Windows 실제 입력으로 확인해야 한다.
- Windows 실제 타이머 창에서 키보드 포커스, 버튼 활성 상태, 창 숨김 중 시간 진행과 긴급 중지 일시정지를 확인해야 한다.
- 단계 전환 Windows 알림과 트레이의 현재 상태·제어 메뉴는 아직 연결하지 않았다.
- Win32 전경 창 판독과 설정 창의 감지 상태는 일반 사용자 데스크톱에서 실제 메모장/브라우저로 확인해야 한다.
- 투명 Kick 창의 실제 비행 위치, 포커스를 빼앗지 않는지와 최소화 충돌 타이밍은 Windows 사용자 세션에서 확인해야 한다.
- 펫 Dragged/Thrown 상태를 Rust 개입 서비스에 알리는 연결은 아직 없어 해당 두 상태의 즉시 취소를 보강해야 한다.

## 다음 작업

1. 실행 중인 dev 앱을 완전히 종료하고 다시 시작해 새 Rust 감시 루프를 로드한다.
2. GAMCHA 옷장에서 `rare_025`를 적용하고 X·Y·크기 슬라이더로 머리 위치에 맞춘 뒤 재시작해 값이 유지되는지 확인한다.
3. 집중을 시작하고 Chrome `YouTube` 규칙을 5초 이상 유지해 왼쪽 Kick 캐릭터와 창 최소화를 확인한다.
4. 보유 중인 얼굴·목·몸·전신 아이템도 같은 방식으로 한 번씩 맞추고 `위치 초기화`를 확인한다.
5. 위치만으로 맞지 않는 복합 세트는 자산 분리/마스킹 대상으로 ID를 기록한다.

## 작업 5 완료 게이트

- Stopped→Focus→ShortBreak/LongBreak→Focus 전환이 동작한다.
- Pause/Resume가 남은 시간을 보존하고 Skip/Stop이 올바르게 동작한다.
- 타이머 창을 숨겨도 종료 시각 기준으로 시간이 진행된다.
- 긴급 중지가 실행 중 타이머를 일시정지한다.
- 상태를 색상뿐 아니라 텍스트로도 표시하고 모든 제어를 키보드로 사용할 수 있다.
- Rust·TypeScript 테스트, Clippy와 production build가 통과한다.

## 작업 3 완료 게이트

- 실제 감자봇만 보이고 창 배경이 완전히 투명하다.
- Idle과 좌·우 Walk가 실제 감자봇 프레임으로 재생된다.
- 단일/다중 모니터 작업 영역을 벗어나 영구 유실되지 않는다.
- 작업표시줄 영역을 침범하지 않는다.
- 자동 테스트, TypeScript 검사와 release exe 빌드가 통과한다.

## 작업 4 완료 게이트

- Dragged가 Idle/Walk를 즉시 중단한다.
- 천천히 놓기와 빠르게 던지기가 구분된다.
- 던진 감자봇이 중력과 화면 경계 반동을 적용받는다.
- 어떤 던지기도 3초 이내 종료되고 화면 밖으로 영구 유실되지 않는다.
- 포인터 속도와 물리 자동 테스트가 통과한다.
