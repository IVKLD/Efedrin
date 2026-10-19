pub const BASE_CSS: &str = "
.sound-card-button {
    padding: 6px;
    border-radius: 12px;
    border: 2px solid transparent;
    min-width: 170px;
    min-height: 86px;
    transition: background-color 180ms ease-in-out, border-color 180ms ease-in-out, box-shadow 180ms ease-in-out;
}

.sound-card-button:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
}

.sound-card-button.is-playing {
    border-color: @accent_color;
    background-color: alpha(@accent_color, 0.18);
    box-shadow: 0 0 14px alpha(@accent_color, 0.45);
}

.badge {
    padding: 2px 8px;
    border-radius: 8px;
    background-color: alpha(currentColor, 0.12);
    font-weight: bold;
    min-width: 58px;
}

.badge.badge-active {
    background-color: @accent_color;
    color: #ffffff;
}

.pill {
    padding: 4px 10px;
    border-radius: 14px;
    background-color: alpha(@accent_color, 0.15);
    color: @accent_color;
}
";
