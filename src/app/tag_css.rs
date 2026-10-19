pub const TAG_CSS: &str = "
.tag-pill {
    padding: 3px 8px;
    border-radius: 8px;
    font-size: 11px;
    font-weight: 600;
    transition: background-color 140ms ease-in-out, color 140ms ease-in-out;
}
.tag-btn-add {
    padding: 2px 7px;
    border-radius: 8px;
    font-size: 11px;
    background-color: transparent;
    border: 1px dashed alpha(currentColor, 0.28);
    color: alpha(currentColor, 0.7);
}
.tag-btn-add:hover {
    border-color: @accent_color;
    color: @accent_color;
    background-color: alpha(@accent_color, 0.12);
}
.tag-chip {
    padding: 3px 8px;
    border-radius: 10px;
    font-size: 12px;
    font-weight: 500;
}
.tag-remove-btn {
    min-width: 18px;
    min-height: 18px;
    padding: 0;
    margin: 0;
    border-radius: 9px;
    background: transparent;
    border: none;
    opacity: 0.65;
}
.tag-remove-btn:hover {
    opacity: 1.0;
    background-color: alpha(currentColor, 0.22);
}
.tag-color-blue { background-color: rgba(53, 132, 228, 0.18); color: #78aeed; border: 1px solid rgba(53, 132, 228, 0.35); }
.tag-color-blue:hover, .tag-color-blue.badge-active { background-color: rgba(53, 132, 228, 0.38); color: #fff; border-color: #3584e4; }
.tag-color-green { background-color: rgba(46, 194, 126, 0.18); color: #57e389; border: 1px solid rgba(46, 194, 126, 0.35); }
.tag-color-green:hover, .tag-color-green.badge-active { background-color: rgba(46, 194, 126, 0.38); color: #fff; border-color: #2ec27e; }
.tag-color-orange { background-color: rgba(255, 120, 0, 0.18); color: #ffbe6f; border: 1px solid rgba(255, 120, 0, 0.35); }
.tag-color-orange:hover, .tag-color-orange.badge-active { background-color: rgba(255, 120, 0, 0.38); color: #fff; border-color: #ff7800; }
.tag-color-purple { background-color: rgba(145, 65, 172, 0.20); color: #dc8add; border: 1px solid rgba(145, 65, 172, 0.38); }
.tag-color-purple:hover, .tag-color-purple.badge-active { background-color: rgba(145, 65, 172, 0.40); color: #fff; border-color: #9141ac; }
.tag-color-red { background-color: rgba(224, 27, 36, 0.18); color: #f66151; border: 1px solid rgba(224, 27, 36, 0.35); }
.tag-color-red:hover, .tag-color-red.badge-active { background-color: rgba(224, 27, 36, 0.38); color: #fff; border-color: #e01b24; }
.tag-color-teal { background-color: rgba(0, 163, 174, 0.18); color: #63e4ec; border: 1px solid rgba(0, 163, 174, 0.35); }
.tag-color-teal:hover, .tag-color-teal.badge-active { background-color: rgba(0, 163, 174, 0.38); color: #fff; border-color: #00a3ae; }
.tag-color-yellow { background-color: rgba(246, 211, 45, 0.18); color: #f9f06b; border: 1px solid rgba(246, 211, 45, 0.35); }
.tag-color-yellow:hover, .tag-color-yellow.badge-active { background-color: rgba(246, 211, 45, 0.38); color: #fff; border-color: #f6d32d; }
.tag-color-pink { background-color: rgba(213, 97, 153, 0.20); color: #f08fc0; border: 1px solid rgba(213, 97, 153, 0.38); }
.tag-color-pink:hover, .tag-color-pink.badge-active { background-color: rgba(213, 97, 153, 0.40); color: #fff; border-color: #d56199; }
.tag-color-indigo { background-color: rgba(98, 91, 219, 0.20); color: #aba5f2; border: 1px solid rgba(98, 91, 219, 0.38); }
.tag-color-indigo:hover, .tag-color-indigo.badge-active { background-color: rgba(98, 91, 219, 0.40); color: #fff; border-color: #625bdb; }
.tag-color-cyan { background-color: rgba(26, 170, 214, 0.18); color: #72d9f7; border: 1px solid rgba(26, 170, 214, 0.35); }
.tag-color-cyan:hover, .tag-color-cyan.badge-active { background-color: rgba(26, 170, 214, 0.38); color: #fff; border-color: #1aaad6; }
";
