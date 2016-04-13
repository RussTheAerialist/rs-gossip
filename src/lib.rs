pub struct GossipDiscovery {
    port: u16,
    heartbeat: u32,
}

impl GossipDiscovery {
    /// Constructs a new `GossipDiscovery`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gossip::GossipDiscovery;
    /// let discovery = GossipDiscovery::new(9999);
    /// ```
    pub fn new(port: u16) -> GossipDiscovery {
        GossipDiscovery {
            heartbeat: 0,
            port: port,
        }
    }

    /// Generate a copy of the current GossipDiscovery with the next
    /// heartbeat
    ///
    /// # Examples
    /// ```
    /// use gossip::GossipDiscovery;
    /// let discovery = GossipDiscovery::new(9999);
    /// let nextBeat = discovery.next();
    /// assert_eq!(discovery.get_heartbeat()+1, nextBeat.get_heartbeat());
    /// ```
    pub fn next(&self) -> GossipDiscovery {
        GossipDiscovery { heartbeat: self.heartbeat + 1, ..*self }
    }

    pub fn get_heartbeat(&self) -> u32 {
        self.heartbeat
    }
}
