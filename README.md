# Efedrin

Саундпад для Linux и PipeWire с одновременным выводом в наушники и виртуальный микрофон.

## Возможности

- Двойной вывод: одновременно играет тебе в наушники и в виртуальный микрофон `efedrin_mic`.
- Сквозной микрофон: возможность подмешивать свой голос в виртуальный микрофон.
- Раздельная регулировка громкости: общая громкость сэмплов (Sound Vol), наушники и микрофон (до 1000%).
- Перемотка по таймлайну кликом по прогресс-бару.
- Горячие клавиши: `Space` или `Esc` — мгновенно заглушить всё.
- Поддерживает форматы mp3, wav, ogg, flac, opus, m4a.

## Установка в NixOS и Home Manager

> **Быстрая установка:** По умолчанию Flake использует готовый предсобранный бинарник (`efedrin-bin`) через `autoPatchelfHook` — сборка Rust на вашей машине **не требуется**, установка занимает пару секунд!

### Вариант 1: Через Home Manager (Flakes)

В `flake.nix` (конфигурации системы или dotfiles):

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

Или в `home.nix` через `home.packages`:

```nix
home.packages = [
  inputs.efedrin.packages.${pkgs.system}.default
];
```

### Вариант 2: В конфигурацию NixOS

В `flake.nix` твоей системы:

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

### Вариант 3: Установка в профиль пользователя (nix profile)

```bash
nix profile install github:IVKLD/Efedrin
```

### Вариант 4: Запуск без установки

```bash
nix run github:IVKLD/Efedrin
```

## Сборка из исходников

Если вы хотите принудительно собрать проект из исходников на своей машине:

```bash
# Через Nix Flake:
nix build .#efedrin-source

# Или через Cargo:
cargo build --release
./target/release/efedrin
```

Для разработки на NixOS:

```bash
nix develop
cargo run
```

## Как использовать в Discord / Telegram / OBS

1. Запусти `efedrin`. В системе появится микрофон `Efedrin Virtual Microphone` (`efedrin_mic`).
2. В настройках звука Discord/Telegram/игры выбери устройство ввода: **Efedrin Virtual Microphone**.
3. В приложении нажми «Choose Folder...» и укажи папку со звуками.
4. Если хочешь говорить своим голосом в тот же микрофон — включи тумблер **Mix Mic** и выбери свой физический микрофон в выпадающем списке.
