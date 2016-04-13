extern crate gossip;

#[test]
fn test_gossip_basic() {
    let dis = gossip::GossipDiscovery::new(9999);
    assert_eq!(dis.get_heartbeat(), 0);
}
