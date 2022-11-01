mod test;
/*  Definition for singly-linked list. */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from(collection: &Vec<i32>) -> ListNode {
        let collection = collection.iter().rev().collect::<Vec<&i32>>();

        let mut list = ListNode {
            val: *collection[0],
            next: None,
        };

        for i in 1..collection.len() {
            let current = collection[i];

            let new_node = ListNode {
                val: *current,
                next: Some(Box::new(list)),
            };

            list = new_node;
        }

        list
    }
}

fn get_value(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut pointer = list;

    let mut vector = Vec::new();

    loop {
        if let Some(item) = pointer {
            vector.push(item.val);
            pointer = &item.next;
        } else {
            break;
        }
    }

    vector
}

pub fn palindrome(head: Option<Box<ListNode>>) -> bool {
    let vector = get_value(&head);

    let vector_string = format!("{:?}", &vector);
    let reversed = vector.clone().into_iter().rev().collect::<Vec<i32>>();
    let reversed_string = format!("{:?}", &reversed);

    let is_palindrome = vector_string == reversed_string;
    is_palindrome
}

fn main() {}
