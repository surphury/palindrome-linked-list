use core::slice::Iter;

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

    fn from(collection: Vec<i32>) -> Box<ListNode> {
        let filled_list = Self::build(None, collection.iter());

        filled_list.expect("Doesn't exists")

        /* let filled_list = Self::build(collection.iter(),) */
        /* let mut list = Some(Box::new(ListNode {
            val: collection[0],
            next: None,
        }));

        let (mut current, mut count) = (list, 0);

        while let Some(mut node) = current {
            node.next = Some(Box::new(ListNode {
                val: collection[count],
                next: None,
            }));
            /* println!("{:#?}", current); */

            current = node.next;
            count += 1;
        } */

        /*  let mut list = Self::new(collection[0], None);
        /* let hola = vec![..&list]; */
        /* println!("{:#?}", hola); */
        let mut pointer = &list.next;
        let mut node = None;
        let mut i = 1;

        let something = loop {
            node = Some(Box::new(Self::new(collection[i], None)));
            pointer = &node;
            pointer = match pointer {
                Some(item) => &item.next,
                None => &None,
            };

            if i == collection.len() - 1 {
                break list;
            }
            i += 1;
        }; */

        /* list.next = Some(Box::new(Self::new(collection[1], None)));
               println!("{:#?}", list);
        */
        /* pointer; */
        /* let mut list = Box::new(Self::new(collection[0], None));
        let mut reference = &Some(list);

        for i in 1..collection.len() - 1 {
            let current = collection[i];

            let node = Some(Box::new(ListNode::new(current, None)));

            match reference {
                Some(mut refer) => {
                    refer.next = node;
                    reference = refer.next;
                }
                None => {}
            };
        }

        list */
        /* list.expect("") */
    }

    fn build(mut list: Option<Box<ListNode>>, mut iterator: Iter<i32>) -> Option<Box<ListNode>> {
        if let Some(value) = iterator.next() {
            match &list {
                Some(item) => {
                    let new_item = Some(Box::new(ListNode::new(*value)));

                    list.expect("msg").next = item.next;

                    list.clone().expect("XD? WTF?").next = new_item;

                    Self::build(list, iterator)
                }
                None => Self::build(Some(Box::new(ListNode::new(*value))), iterator),
            }
        } else {
            list
        }

        /* if let Some(item) = iterator.next() {
            if let Some(list) = &mut list {
                list.next = Some(Box::new(ListNode {
                    val: item.clone(),
                    next: None,
                }));
            }

            Self::build(iterator, list)
        } else {
            list
        } */
        /*  let hola = vec![2123, 21].iter();
        collection.iter().for_each(|number| {}) */
        /* for i in 0..collection.len() - 1 {
            let current = collection[i];
            let next = collection[i + 1];

            list.expect("XD").next = Some(Box::new(ListNode {
                val: current,
                next: None,
            }));

            collection.pop();

            Self::build(collection, list.expect("XD2").next);
        }
        list */
    }

    /* fn from(collection: Vec<i32>) -> ListNode {
        let mut initial = ListNode::new(collection[0],
            next {
            collection.map(|val| {
                ListNode::new(val, next)
            })

        });

        ListNode::build(collection, Some(Box::new(initial)));

        /* let mut last_item = initial.next;

            for i in 1..collection.len() {
                /*  println!("{:#?}", i); */
                let current = collection[i];

                let mut new_node = Some(Box::new(ListNode {
                    val: current,
                    next: None,
                }));

                last_item = Some(Box::new(last_item.next));
            }
            initial
        } */
        initial
    } */

    /* fn build(collection: Vec<i32>) -> ListNode {
        fn get(value: i32, list: ListNode) -> ListNode {
            if list.next.is_none() {
                list
            } else {
                get(value, list)
            }
        }
    } */
}

fn get_value(params: (Vec<i32>, &Option<Box<ListNode>>)) -> (Vec<i32>, Option<Box<ListNode>>) {
    let (list, item) = params;

    let next = match &item {
        Some(item) => &item.next,
        None => &None,
    };

    match item {
        Some(item) => {
            let list = list.into_iter().chain(vec![item.val]).collect();
            get_value((list, next))
        }
        None => (list, None),
    }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let (vector, _list) = get_value((Vec::new(), &head));
    let reversed = vector.clone().into_iter().rev().collect::<Vec<i32>>();

    let is_palindrome = vector.starts_with(&reversed);
    is_palindrome
}

fn main() {
    /*  ListNode::new(2, None).iter(); */
    let list = ListNode::from(vec![1, 2, 2, 2, 2, 2, 2, 2, 1]);
    println!("{:#?}", list);
}
