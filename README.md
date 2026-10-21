# Efedrin

<p align="center">
  <strong>Современный саундпад для Linux и PipeWire с двойной маршрутизацией и поддержкой виртуального микрофона.</strong>
</p>

<p align="center">
  <a href="https://github.com/IVKLD/Efedrin/releases"><img src="https://img.shields.io/github/v/release/IVKLD/Efedrin?style=flat-square&color=blue" alt="Release"></a>
  <a href="https://github.com/IVKLD/Efedrin/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-green?style=flat-square" alt="License"></a>
  <a href="https://pipewire.org"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20PipeWire-orange?style=flat-square" alt="Platform"></a>
</p>

---

## Возможности

- 🎧 **Двойной вывод (Dual Routing):** звук играет одновременно в ваши наушники и в виртуальный микрофон `efedrin_mic`.
- 🎙️ **Сквозной микрофон (Mic Passthrough):** подмешивайте свой физический голос в виртуальный микрофон в один клик.
- 🔊 **Раздельная регулировка громкости:**
  - Общая громкость сэмплов (**Sound Vol**).
  - Громкость в наушниках (**Headphones**).
  - Громкость в микрофон (**Microphone**) с запасом усиления до **1000%**.
- 🏷️ **Теги и организация звуков:** группировка звуков по категориям и быстрый фильтр по тегам.
- ⚡ **Мгновенная асинхронная загрузка:** файлы загружаются в фоне без зависаний UI, длительность треков определяется параллельно.
- 🔄 **Автообновление библиотеки:** программа отслеживает выбранную папку в реальном времени — новые файлы появляются на панели мгновенно.
- ⏱️ **Перемотка по таймлайну:** кликабельный прогресс-бар для точного позиционирования в треке.
- 🛑 **Экстренное глушение (Panic Mute):** `Space` или `Esc` мгновенно останавливают воспроизведение всех звуков.
- 🎵 **Широкая поддержка форматов:** `mp3`, `wav`, `ogg`, `flac`, `opus`, `m4a`.

---

## Установка в NixOS и Home Manager

> **Быстрая установка:** По умолчанию Flake использует готовый предсобранный бинарник (`efedrin-bin`) через `autoPatchelfHook` — сборка Rust на вашей машине **не требуется**, установка занимает пару секунд!

### Вариант 1: Home Manager (Flakes)

Добавьте репозиторий в `inputs` вашего `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    efedrin.url = "github:IVKLD/Efedrin";
  };

  outputs = { nixpkgs, home-manager, efedrin, ... }: {
    # В модулях home-manager:
    homeConfigurations.youruser = home-manager.lib.homeManagerConfiguration {
      modules = [
        efedrin.homeManagerModules.default
        {
          programs.efedrin.enable = true;
        }
      ];
    };
  };
}
```

Или просто добавьте пакет в `home.packages` внутри вашего `home.nix`:

```nix
home.packages = [
  inputs.efedrin.packages.${pkgs.system}.default
];
```

---

### Вариант 2: NixOS (системный модуль)

В системном `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    efedrin.url = "github:IVKLD/Efedrin";
  };

  outputs = { nixpkgs, efedrin, ... }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      modules = [
        efedrin.nixosModules.default
        {
          programs.efedrin.enable = true;
        }
      ];
    };
  };
}
```

---

### Вариант 3: Установка в пользовательский профиль (`nix profile`)

```bash
nix profile install github:IVKLD/Efedrin
```

---

### Вариант 4: Быстрый запуск без установки

```bash
nix run github:IVKLD/Efedrin
```

---

## Сборка из исходников

Если вы хотите собрать приложение вручную:

### Через Nix Flake:
```bash
nix build .#efedrin-source
./result/bin/efedrin
```

### Через Cargo:
Потребуются установленные библиотеки: `gtk4`, `libadwaita`, `pipewire`, `alsa-lib`, `ffmpeg`.

```bash
cargo build --release
./target/release/efedrin
```

Для разработки на NixOS:
```bash
nix develop
cargo run
```

---

## Настройка в Discord, Telegram, OBS и играх

1. **Запустите `efedrin`.** Приложение автоматически зарегистрирует виртуальное устройство `Efedrin Virtual Microphone` (`efedrin_mic`) в PipeWire.
2. **Выберите микрофон в программах:** В настройках голосовой связи Discord, Telegram, OBS или игры укажите устройство ввода:
   ```
   Efedrin Virtual Microphone
   ```
3. **Выберите папку со сэмплами:** В Efedrin нажмите **«Choose Folder...»** и выберите директорию со звуками.
4. **Сквозной голос:** Чтобы общаться своим голосом параллельно со сэмплами, включите тумблер **Mix Mic** и выберите физический микрофон в выпадающем списке.
5. **Тегирование:** Нажмите правой кнопкой мыши по карточке звука, чтобы назначить категорию или тег для быстрой фильтрации.

---

## Горячие клавиши

| Клавиша | Действие |
| :--- | :--- |
| `Space` / `Esc` | Мгновенно остановить воспроизведение всех звуков |
| Клик по прогресс-бару | Перемотка звука в указанное место |
| ПКМ по звуковой карте | Открыть контекстное меню назначения тегов |
