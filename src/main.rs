use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let mut s = String::from("");

    let (tx, rx) = channel();
    thread::spawn(move || {
      let s = one(s);
      tx.send(s).unwrap();
    });
 
    let (tx2, rx2) = channel();
    thread::spawn(move || {
      let mut s = rx.recv().unwrap();
      two(&mut s);
      tx2.send(s).unwrap();
    });

    let (tx3, rx3) = channel();
    thread::spawn(move || {
      let mut s = rx2.recv().unwrap();
      s = three(&s);
      tx3.send(s).unwrap();
    });

    s = rx3.recv().unwrap();

    println!("Output: {}", s);

    println!("{}", &s[..3]);
    println!("{}", &s[3..6]);
    println!("{}", &s[6..]);
}

fn one(s: String) -> String {
  let mut out = s;
  out.push_str("one");
  out
}

fn two(s: &mut String) {
  s.push_str("two");
}

fn three(s: &String) -> String {
  let mut out = s.clone();
  out.push_str("three");
  out
}
