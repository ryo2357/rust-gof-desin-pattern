// 制御するボタンが一つだけついた電子サイコロ
// ボタンを押す、電源ON & サイコロ回転開始
// ボタンを押す、サイコロ停止(出目が決まる)
// ボタンを押す、電源OFF、最初に戻る.
use std::collections::HashMap;

// 1. State(状態)
// 状態を表すクラス
// 状態毎に振舞いが異なるメソッドのインタフェースを定義します。
#[derive(Debug, Eq, PartialEq, Hash)]
enum StateDice {
    PowerOn,
    StopDice,
    PowerOff
}


trait State {
    // fn on_press_button(&self, &mut StateContext);
    fn on_press_button(&self, context: &mut StateContext);
}

// 2. ConcreteStateA・B(具体的な状態)
// 「State」のインタフェースを実装し、具体的な状態を、「1クラス」 = 「1状態」 で定義します。
// 1つ状態を表すのに複数のオブジェクトは必要ないため、「Singleton」パターンを適用します。
struct StatePowerOn;
impl State for StatePowerOn {
    fn on_press_button(&self, context: &mut StateContext)
    {
        // Something to do for turning on the dice.
        println!("Power on and Shake the dice.");

        context.set_state(StateDice::StopDice);
    }
}

struct StateStop;
impl State for StateStop {
    fn on_press_button(&self, context: &mut StateContext)
    {
        // Something to do for turning on the dice.
        println!("Stopping the dice.");

        context.set_dice_number(4);

        context.set_state(StateDice::PowerOff);
    }
}

struct StatePowerOff;
impl State for StatePowerOff {
    fn on_press_button(&self, context: &mut StateContext)
    {
        // Something to do for turning on the dice.
        println!("Power off.");

        context.set_state(StateDice::PowerOn);
    }
}

// 3. Context(状況判断)
// 現在の状態(「ConcreteStateA」か「ConcreteStateB」)を保持します。
// 利用者へのインタフェースを定義します。
// 状態を変更するメソッドを定義します。(状態の変更は、「ConcreteState」が次ぎの状態として相応しいものを判断し、この状態変更メソッドを呼出すことによって行います。)
#[derive(Debug)]
struct StateContext {
    number: Option<u8>,
    current_state: StateDice,
}

impl StateContext {
    fn new() -> StateContext
    {
        StateContext {
            number: None,
            current_state: StateDice::PowerOn,
        }
    }

    fn set_state(&mut self, s: StateDice)
    {
        self.current_state = s;
    }

    fn set_dice_number(&mut self, n :u8)
    {
        self.number = Some(n)
    }

    // fn press_button<'a>(&mut self, hmap: &HashMap<StateDice, Box<State + 'a>>)
    fn press_button<'a>(&mut self, hmap: &HashMap<StateDice, Box<dyn State + 'a>>)
    {
        let b = hmap.get(&self.current_state).unwrap();
        // ハッシュマップに現在の状態をを入力⇒コマンドのBoxが返ってくる
        b.on_press_button(self);
        // 返ってきたコマンドを実行

        // ここで、Stateの確認、引数からの処理の決定をすればよい
    }
}

// 4. Client(利用者)
// 「State」パターンを適用したクラスを用い処理を行います。
pub fn main() {
    let mut hmap = HashMap::new();
    // hmap.insert(StateDice::PowerOn,  Box::new(StatePowerOn)    as Box<State>);
    // hmap.insert(StateDice::StopDice, Box::new(StateStop)       as Box<State>);
    // hmap.insert(StateDice::PowerOff, Box::new(StatePowerOff)   as Box<State>);
    hmap.insert(StateDice::PowerOn,  Box::new(StatePowerOn)    as Box<dyn State>);
    hmap.insert(StateDice::StopDice, Box::new(StateStop)       as Box<dyn State>);
    hmap.insert(StateDice::PowerOff, Box::new(StatePowerOff)   as Box<dyn State>);
    let hmap = &hmap;

    let mut context = StateContext::new();

    context.press_button(hmap);
    println!("{:?}", context);
    context.press_button(hmap);
    println!("{:?}", context);
    context.press_button(hmap);
    println!("{:?}", context);
}