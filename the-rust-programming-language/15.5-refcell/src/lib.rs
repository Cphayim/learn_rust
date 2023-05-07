pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You have used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You have used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // 将 Vec<String> 存放在 RefCell<T>> 中
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 这样就可以通过 RefCell<T> 获取到 Vec<String> 的可变借用
            self.sent_messages.borrow_mut().push(String::from(message));

            // 当使用 RefCell<T> 违反借用规则时，将会导致运行时 panic
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // 这里只需要不可变借用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

        // RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
        // 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。
        // 当 Ref<T> 值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
    }
}
