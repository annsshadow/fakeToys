#[cfg(test)]
mod tests {
    use crate::OrganizationBindStore;

    #[test]
    fn test_bind_store_create_and_take_confirmed() {
        let store = OrganizationBindStore::new();
        let meta = store.create("person-1", "qywx_123", Some("Alice".to_string()));
        assert!(store.take_confirmed(&meta).is_none()); // not confirmed yet

        store.confirm(&meta);
        let entry = store.take_confirmed(&meta);
        assert!(entry.is_some());
        let e = entry.unwrap();
        assert_eq!(e.person_unique, "person-1");
        assert_eq!(e.external_user_id, "qywx_123");
        assert_eq!(e.external_name, Some("Alice".to_string()));
        // one-time use
        assert!(store.take_confirmed(&meta).is_none());
    }

    #[test]
    fn test_bind_store_unknown_meta() {
        let store = OrganizationBindStore::new();
        assert!(store.take_confirmed("nonexistent").is_none());
        assert!(!store.confirm("nonexistent"));
    }

    #[test]
    fn test_bind_store_create_generates_unique_meta() {
        let store = OrganizationBindStore::new();
        let m1 = store.create("p1", "ext1", None);
        let m2 = store.create("p2", "ext2", None);
        assert_ne!(m1, m2);
    }
}
