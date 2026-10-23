slint::slint! {
    import { Button, Slider, SpinBox } from "std-widgets.slint";
    export component ReaderWindow inherits Window {
        in property <[string]> words;
        private property <int> word-count: root.words.length;
        in-out property <string> load-error;
        callback open-file();
        public function reset-document() {
            root.playing = false;
            root.position = 0;
            root.finished = false;
        }
        callback show-about();
        callback quit();
        pure callback group-text([string], int, int) -> string;
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
        forward-focus: play-button;
        title: "Fasteyes";
        preferred-width: 800px;
        preferred-height: 400px;
        background: white;
        MenuBar {
            Menu {
                title: "File";
                MenuItem {
                    title: "Open File...";
                    shortcut: @keys(Control + O);
                    activated => { root.playing = false; root.open-file(); }
                }
                MenuItem { title: "Quit"; activated => { root.quit(); } }
            }
            Menu {
                title: "View";
                MenuItem { title: "View options"; enabled: false; }
            }
            Menu {
                title: "History";
                MenuItem { title: "Recent documents"; enabled: false; }
            }
            Menu {
                title: "Profiles";
                MenuItem { title: "Manage profiles"; enabled: false; }
            }
            Menu {
                title: "Help";
                MenuItem { title: "About Fasteyes"; activated => { root.show-about(); } }
            }
        }
        reader-focus := FocusScope {
            focus-on-click: false;
            focus-on-tab-navigation: false;
            key-pressed(event) => {
                if (!self.has-focus && !play-button.has-focus) || root.word-count == 0
                    || event.modifiers.control || event.modifiers.alt
                    || event.modifiers.shift || event.modifiers.meta {
                    return reject;
                }
                if event.text == Key.LeftArrow {
                    if root.position == 0 { return accept; }
                    root.position = max(0, root.position - words-per-display.value);
                } else if event.text == Key.RightArrow {
                    if root.position + root.displayed-count >= root.word-count { return accept; }
                    root.position += words-per-display.value;
                } else {
                    return reject;
                }
                root.finished = false;
                if root.playing { playback.restart(); }
                return accept;
            }
            VerticalLayout {
                spacing: 0px;
                Text {
                    vertical-stretch: 1;
                    text: root.group-text(root.words, root.position, root.displayed-count);
                    wrap: word-wrap;
                    color: black;
                    font-family: "sans-serif";
                    font-size: 48px;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                    TouchArea { clicked => { reader-focus.focus(); } }
                }
                Text {
                    text: root.load-error;
                    visible: root.load-error != "";
                    color: #b00020;
                    wrap: word-wrap;
                }
                navigation := Slider {
                    accessible-label: "Reading position";
                    minimum: 0;
                    maximum: max(0, root.word-count - 1);
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
                    play-button := Button {
                        enabled: root.word-count > 0;
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
    export component AboutWindow inherits Window {
        in property <string> version;
        callback dismiss();
        title: "About Fasteyes";
        min-width: 320px;
        max-width: 320px;
        min-height: 160px;
        max-height: 160px;
        changed maximized => { if self.maximized { self.maximized = false; } }
        changed full-screen => { if self.full-screen { self.full-screen = false; } }
        background: white;
        forward-focus: close-button;
        VerticalLayout {
            padding: 16px;
            spacing: 12px;
            Text { text: "Fasteyes " + root.version; color: black; font-size: 20px; }
            Text { text: "A speed reader that presents words from a document."; color: black; wrap: word-wrap; }
            close-button := Button { text: "Close"; clicked => { root.dismiss(); } }
        }
    }
}
