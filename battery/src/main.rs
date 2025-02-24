use battery::*;
use crossterm::{cursor, event, queue, style, terminal};
use std::time::Duration;

const FONT: [[[u8; 5]; 3]; 11] = [
    [
        [1, 1, 1, 1, 1], // 0
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [0, 0, 0, 0, 0], // 1
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0],
    ],
    [
        [1, 0, 1, 1, 1], // 2
        [1, 0, 1, 0, 1],
        [1, 1, 1, 0, 1],
    ],
    [
        [1, 0, 1, 0, 1], // 3
        [1, 0, 1, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 0, 0], // 4
        [0, 0, 1, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 0, 1], // 5
        [1, 0, 1, 0, 1],
        [1, 0, 1, 1, 1],
    ],
    [
        [1, 1, 1, 1, 1], // 6
        [1, 0, 1, 0, 1],
        [1, 0, 1, 1, 1],
    ],
    [
        [1, 0, 0, 0, 0], // 7
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 1, 1], // 8
        [1, 0, 1, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 0, 1], // 9
        [1, 0, 1, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 0, 0, 1, 1], // %
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 1],
    ],
];

/*
fn make_figlet_output(text: String) -> Result<String, String> {
    if !cfg!(target_os = "windows") {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("figlet {}", text))
            .output()
            .expect("failed to execute process");
        match std::str::from_utf8(&output.stdout) {
            Ok(output_string) => Ok(output_string.to_string()),
            Err(_) => Err("cannot parse stdout to string".to_string()),
        }
    } else {
        Err("windows not supported".to_string())
    }
}
*/

fn fontify(mut percentage: u8) -> Vec<[[u8; 5]; 3]> {
    let mut font: Vec<[[u8; 5]; 3]> = Vec::new();
    loop {
        font.push(FONT[percentage as usize % 10]);
        percentage = percentage.checked_div(10).unwrap();
        if percentage <= 0 {
            break;
        }
    }
    font.reverse();
    font.push(FONT[10]);
    font
}

fn main() {
    use cursor::{Hide, MoveTo, Show};
    use event::{poll, read, Event, KeyCode, KeyModifiers};
    use std::io::Write;
    use style::{Color, Print, SetForegroundColor};
    use terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    };
    let mut stdout = std::io::stdout();

    let mut perc: u8 = 101;
    let mut stat: bool = false;
    let mut width: u16 = 0;
    let mut height: u16 = 0;

    queue!(stdout, Hide, EnterAlternateScreen,).unwrap();
    stdout.flush().unwrap();
    enable_raw_mode().unwrap();

    loop {
        let acpi_output: String = get_shell_output("upower -d | grep percentage").unwrap();

        let new_perc = get_correct_percentage(&acpi_output).unwrap();
        let new_stat = get_charging_status(&acpi_output);

        let (new_width, new_height) = size().unwrap();
        if (perc != new_perc)
            || (stat != new_stat)
            || (width != new_width)
            || (height != new_height)
        {
            perc = new_perc;
            stat = new_stat;
            width = new_width;
            height = new_height;

            let font = fontify(perc);
            let start_y = (height - 5).checked_div(2).unwrap();
            let start_x = (width - 2 * (3 * font.len() as u16 + font.len() as u16 - 1))
                .checked_div(2)
                .unwrap();

            queue!(stdout, Clear(ClearType::All)).unwrap();
            for (i, character) in font.iter().enumerate() {
                for (j, column) in character.iter().enumerate() {
                    for (k, cell) in column.iter().enumerate() {
                        queue!(
                            stdout,
                            MoveTo(
                                start_x + 2 * (3 + 1) * i as u16 + 2 * j as u16,
                                start_y + k as u16
                            ),
                            SetForegroundColor(if stat {
                                Color::Green
                            } else {
                                if perc > 35 {
                                    Color::White
                                } else {
                                    // if !stat {
                                    //     _ = get_shell_output("swaynag -t warning -m \"BATTERY LOW\"").unwrap();
                                    // }
                                    Color::Red
                                }
                            }),
                            Print(if *cell == 1 { "██" } else { "  " }),
                            //Print(col.iter().map(|n| if *n == 1 { "##" } else { "  " })
                            //      .collect::<Vec<&str>>().join("")),
                        )
                        .unwrap();
                    }
                }
            }

            stdout.flush().unwrap();
        }

        //println!("{} {}", percentage, if charging { "Charging" } else { "Discharging" });
        //println!("{:?}", font);
        //sleep(Duration::from_secs(1));
        if poll(Duration::from_secs(1)).unwrap() {
            if let Event::Key(key) = read().unwrap() {
                if let KeyCode::Esc = key.code {
                    break;
                } else if let KeyCode::Char('q') = key.code {
                    break;
                } else if let KeyCode::Char('c') = key.code {
                    if key.modifiers == KeyModifiers::CONTROL {
                        break;
                    }
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    queue!(stdout, Show, LeaveAlternateScreen,).unwrap();
    stdout.flush().unwrap();
}
