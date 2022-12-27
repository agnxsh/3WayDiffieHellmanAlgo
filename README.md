<h1>3-Way Diffie Hellman Key Exchange Algorithm</h1>

<h5>Elliptic curves are used fairly extensively in public key encryption (such as in Bitcoin and Tor). A BN-curve (Barreto-Naehrig curve) defines an elliptic curve which can be used for pairings that allow for a high security and efficiency level. This page implements the tripartite Diffie-Hellman algorithm and where Bob, Alice and Carol can share a secret key. In this case we will not be using crypto pairing, but have two rounds of exchange. In this case we have a curve (G1) and a generator point (G), and Bob, Alice and Carol determine their private key value (a, b and c). Next they exchange their public key values of aG, bG and cG, and go through two rounds of exchange, to eventually end up with abcG.</h5>

<h4>Why are we particularly using the BN-Curve (Barreto-Naehrig curve) to vary the numbers?</h4>

<h5>Rust gives a robust library for BN-Curves to support implementations of Group Theory and other concepts of Abstract Algebra</h5>

We generate a scalar value of the private key, and then an (x,y) point on the G1 curve:

0422971d5eb242dfa51c74585cfbca5e8e7eecb19e9b33220761b1401320b343951e0b84773b4a92de731800374ec40d1f44f8b726e42ec6c2bcb8b24f766a33d7
In this case we have:

04 22971d5eb242dfa51c74585cfbca5e8e7eecb19e9b33220761b1401320b34395, 1e0b84773b4a92de731800374ec40d1f44f8b726e42ec6c2bcb8b24f766a33d7
and where 04 identifies a point, and we then get an x and a y value (64 bytes each - 256 bits).

A sample run is:

Base point "0400000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002"

Alice priv: "0dc7e1f2bfd65268bdc1157a157cbcd2f022d72cfaac32b02279e2b699089374"

Bob priv "2dc0c0c8893736c98d1806733f5fb2ac9beecf95e6d03e1ad52b2784447f3b59"

Carol priv "1b258c15e22c29e22f97f55f623b7bb9189adf78ed9b88cde3b8a99b0c74d245"

Alice pub: "041eb34aa5b9f67c301ba5a5c2ff12d5c1a2edfd63ac8984e9186857e2e7822437284b4cc8811d925a21392e29c06324c4b430fb9361a6473def8701cb28c866f9"

Bob pub "042f70b1e97911e70ae8ae38dcc2e4417f976131411265eda478c980e61d982c431f102a0e7eb05a027d76c8ff858f7ef7c07316533b3745fde5ee39abe3b351f3"

Carol pub "04224582dd038b27abd06abed8565ca1a5d7b25377297154c81183032370df37ff168828953a2e81583762495f4587c0a51b01f2b10459409f19de6eb3e5f5a036"

Alice share: "041609af48251c0eb556415c5884ef10c11ce59586767e76033e9d5192a22325902e90747a66c2b2b5133ae390b76634348b1ef9ac66b5ce6650e93a143c8afe88"

Bob share "041609af48251c0eb556415c5884ef10c11ce59586767e76033e9d5192a22325902e90747a66c2b2b5133ae390b76634348b1ef9ac66b5ce6650e93a143c8afe88"

Carol share "041609af48251c0eb556415c5884ef10c11ce59586767e76033e9d5192a22325902e90747a66c2b2b5133ae390b76634348b1ef9ac66b5ce6650e93a143c8afe88"

<h3>Related: An Easy To Understand 2-Way Diffie Hellman Algorithm</h3>

![image](https://user-images.githubusercontent.com/80243668/209595637-740ba4d0-e4fc-4945-ba7e-d17fa0ca2b37.png)

