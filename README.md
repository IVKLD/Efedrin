# Efedrin

Саундпад для Linux и PipeWire с одновременным выводом в наушники и виртуальный микрофон.

## Возможности

- Двойной вывод: одновременно играет тебе в наушники и в виртуальный микрофон `efedrin_mic`.
- Сквозной микрофон: возможность подмешивать свой голос в виртуальный микрофон.
- Независимая громкость для наушников и микрофона (до 1000%).
- Перемотка по таймлайну кликом по прогресс-бару.
- Горячие клавиши: `Space` или `Esc` — мгновенно заглушить всё.
- Поддерживает форматы mp3, wav, ogg, flac, opus, m4a.

## Установка в NixOS

### Вариант 1: Через Flake Inputs (рекомендуется)

В `flake.nix` твоей системы:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    efedrin.url = "github:IVKLD/Efedrin";
  };

  outputs = { self, nixpkgs, efedrin, ... }: {
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

Или просто добавить пакет в `environment.systemPackages`:

```nix
environment.systemPackages = [
  inputs.efedrin.packages.${pkgs.system}.default
];
```

### Вариант 2: Запуск без установки

```bash
nix run github:IVKLD/Efedrin
```

### Вариант 3: Установка через nix profile

```bash
nix profile install github:IVKLD/Efedrin
```

## Сборка из исходников

Нужны `rust`, `cargo`, `gtk4`, `libadwaita`, `pipewire`, `ffmpeg`.

```bash
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
