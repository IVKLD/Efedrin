# Efedrin Soundpad (Soundpad Linux)

> **Production-Ready PipeWire Soundpad for Linux & NixOS**  
> Воспроизведение звуковых эффектов с одновременной двойной маршрутизацией в наушники пользователя и в виртуальный микрофон PipeWire (`efedrin_mic`) для трансляции в Discord, Telegram, Steam, OBS и игры без задержек и помех.

---

## 🌟 Основные возможности (Core Features)

1. **Одновременный вывод аудио (Dual Audio Routing)**:
   - **Headphones Output**: звук воспроизводится локально в текущее устройство по умолчанию (наушники/колонки).
   - **Virtual Microphone Output**: звук одновременно поступает в созданный виртуальный микрофон `efedrin_mic`. Собеседники в голосовых чатах слышат чистый звук.
2. **Раздельная регулировка громкости и мгновенный Stop**:
   - Независимый ползунок громкости в наушниках (0% – 150%).
   - Независимый ползунок громкости в виртуальный микрофон (0% – 150%).
   - Мгновенная кнопка **Stop All** (горячие клавиши: `Space` или `Esc`), прерывающая все активные аудиопотоки.
3. **Автоматический жизненный цикл PipeWire нод (Zero-Leak)**:
   - При запуске приложение автоматически разворачивает виртуальный Sink (`efedrin_soundpad`) и Source (`efedrin_mic`) через `pw-loopback`.
   - При закрытии приложения или сигнале завершения (SIGINT/SIGTERM) все ноды гарантированно удаляются без остаточных висящих процессов.
4. **Микширование реального микрофона (Mic Passthrough)**:
   - Опциональный переключатель **"Mix Real Mic"** в интерфейсе: автоматически связывает физический микрофон пользователя с виртуальным через `pw-link`. Вы можете одновременно говорить в микрофон и проигрывать звуки через одно виртуальное устройство.
5. **Современный UI (Libadwaita / GTK4)**:
   - Адаптивная сетка карточек звуков с мгновенным поиском/фильтрацией.
   - Сканирование каталогов с поддержкой форматов `.mp3`, `.wav`, `.ogg`, `.flac`, `.opus`, `.m4a`.
   - Правый клик по карточке — мгновенный переход к файлу в системном файловом менеджере (`xdg-open`).
   - Автоматическое сохранение настроек в `~/.config/efedrin/config.json`.

---

## 🏗 Архитектура проекта

```
efedrin/
├── Cargo.toml                  # Манифест Rust со всеми зависимостями
├── Cargo.lock                  # Зафиксированные версии зависимостей
├── flake.nix                   # Nix Flake (пакет + среда devShell)
├── shell.nix                   # Классический nix-shell
├── resources/
│   ├── efedrin.desktop         # Десктопная запись FreeDesktop
│   └── efedrin.svg             # Векторная иконка приложения
├── sample_sounds/              # Демонстрационные звуковые файлы (.wav)
├── src/
│   ├── main.rs                 # Точка входа программы
│   ├── app.rs                  # Жизненный цикл GTK/Adwaita и перехват сигналов Unix
│   ├── config.rs               # Сериализация/десериализация конфигурации (JSON)
│   ├── error.rs                # Типы ошибок и Result (строгая обработка без паник)
│   ├── audio/
│   │   ├── mod.rs
│   │   ├── pipewire.rs         # Управление pw-loopback, нодами PipeWire и pw-link
│   │   └── engine.rs           # Двойная маршрутизация потоков через pw-play и остановка
│   ├── sound/
│   │   ├── mod.rs
│   │   └── library.rs          # Модель SoundItem, рекурсивный сканер и поиск
│   └── ui/
│       ├── mod.rs
│       ├── header.rs           # AdwHeaderBar с кнопкой Stop All и статусом efedrin_mic
│       ├── controls.rs         # Панель выбора папки, поиска, ползунков громкости и микшера
│       ├── sound_card.rs       # Карточка звука с бэйджем формата и контекстным меню
│       ├── sound_grid.rs       # Адаптивная сетка FlowBox и экран пустого состояния
│       └── window.rs           # Главное окно AdwApplicationWindow и связывание состояния
└── tests/
    └── library_test.rs         # Интеграционный тест сканирования звуков
```

---

## 🚀 Сборка и запуск

### Вариант 1: Сборка через Nix Flakes (Рекомендуется для NixOS / Nix)

```bash
# Сборка production-пакета
nix build

# Запуск собранного бинарника
./result/bin/efedrin

# Или запуск без сборки в текущую директорию:
nix run
```

Для входа в среду разработки с готовыми зависимостями:
```bash
nix develop
cargo run
```

### Вариант 2: Сборка в любом Linux-дистрибутиве (Ubuntu, Fedora, Arch)

1. **Системные зависимости**:
   - Ubuntu / Debian:
     ```bash
     sudo apt install build-essential pkg-config libgtk-4-dev libadwaita-1-dev libpipewire-0.3-dev libasound2-dev pipewire pipewire-bin
     ```
   - Arch Linux:
     ```bash
     sudo pacman -S base-devel pkgconf gtk4 libadwaita pipewire alsa-lib
     ```
   - Fedora:
     ```bash
     sudo dnf install gcc pkgconf-pkg-config gtk4-devel libadwaita-devel pipewire-devel alsa-lib-devel
     ```

2. **Сборка через Cargo**:
   ```bash
   cargo build --release
   ./target/release/efedrin
   ```

---

## 🧪 Тестирование и проверка работоспособности

1. **Запуск интеграционных тестов**:
   ```bash
   cargo test
   ```

2. **Проверка создания виртуального микрофона**:
   При запуске `efedrin` выполните в отдельном терминале:
   ```bash
   pw-link -io | grep efedrin
   ```
   Вы увидите зарегистрированные ноды:
   - `efedrin_mic:capture_FL` / `capture_FR` (Virtual Microphone Source)
   - `efedrin_soundpad:playback_FL` / `playback_FR` (Soundpad Sink)

3. **Использование в Discord / Telegram / OBS / играх**:
   - Откройте настройки голосового чата (например, Discord -> Voice & Video -> Input Device).
   - Выберите устройство: **"Efedrin Virtual Microphone"**.
   - Нажмите любую кнопку звука в приложении: звук одновременно зазвучит у вас в наушниках и будет транслироваться вашим собеседникам.
