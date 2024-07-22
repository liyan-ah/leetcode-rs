impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut e = std::collections::HashSet::new();
        for str_s in emails.iter() {
            let items: Vec<_> = str_s.split('@').collect();
            assert_eq!(items.len(), 2);
            let mut local = items[0];
            let domain = items[1];
            let items: Vec<_> = local.split('+').collect();
            local = items[0];
            let local = local.replace(".", "");
            e.insert((local, domain));
        }

        e.len() as i32
    }
}
