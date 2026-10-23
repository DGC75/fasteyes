slint::slint! {
    import { Button, Slider, SpinBox } from "std-widgets.slint";
    export component ReaderWindow inherits Window {
        in property <int> word-count;
        pure callback group-text(int, int) -> string;
        private property <int> position: 0;
        changed position => { navigation.value = root.position; }
        private property <bool> playing: false;
        private property <bool> finished: false;
        private property <int> displayed-count: min(words-per-display.value, root.word-count - root.position);

        playback := Timer {
            interval: 60000ms * root.displayed-count / wpm.value;
            running: root.playing;
            triggered => {
                if root.position + root.displayed-count >= root.word-count {
                    root.playing = false;
                    root.finished = true;
                } else {
                    root.position += root.displayed-count;
                }
            }
        }
        title: "Fasteyes";
        preferred-width: 800px;
        preferred-height: 400px;
        background: white;
        VerticalLayout {
            spacing: 0px;
            Text {
                vertical-stretch: 1;
                text: root.group-text(root.position, root.displayed-count);
                wrap: word-wrap;
                color: black;
                font-family: "sans-serif";
                font-size: 48px;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
            navigation := Slider {
                accessible-label: "Reading position";
                minimum: 0;
                maximum: root.word-count - 1;
                step: 1;
                enabled: root.word-count > 1;
                changed(value) => {
                    root.position = round(value);
                    root.finished = false;
                    if root.playing { playback.restart(); }
                }
            }
            HorizontalLayout {
                vertical-stretch: 0;
                padding: 16px;
                spacing: 12px;
                HorizontalLayout {
                    horizontal-stretch: 1;
                    min-width: 0px;
                    preferred-width: 0px;
                    alignment: center;
                    spacing: 12px;
                    Text {
                        text: "Words per display";
                        color: black;
                        vertical-alignment: center;
                    }
                    words-per-display := SpinBox {
                        accessible-label: "Words per display";
                        minimum: 1;
                        maximum: 5;
                        value: 1;
                        step-size: 1;
                        changed value => { if root.playing { playback.restart(); } }
                    }
                }
                Button {
                    width: 48px;
                    height: 48px;
                    accessible-label: root.finished ? "Restart" : (root.playing ? "Pause" : "Play");
                    icon: root.finished ? @image-url("../assets/restart.svg")
                        : (root.playing ? @image-url("../assets/pause.svg") : @image-url("../assets/play.svg"));
                    clicked => {
                        if root.finished {
                            root.position = 0;
                            root.finished = false;
                        }
                        root.playing = !root.playing;
                    }
                }
                HorizontalLayout {
                    horizontal-stretch: 1;
                    min-width: 0px;
                    preferred-width: 0px;
                    alignment: center;
                    spacing: 12px;
                    Text {
                        text: "WPM";
                        color: black;
                        vertical-alignment: center;
                    }
                    wpm := SpinBox {
                        accessible-label: "WPM";
                        minimum: 60;
                        maximum: 1200;
                        value: 300;
                        step-size: 1;
                        changed value => { if root.playing { playback.restart(); } }
                    }
                }
            }
        }
    }
}
