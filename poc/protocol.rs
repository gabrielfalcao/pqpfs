/// DRAFT Exchange Protocol

use crate::models::RSAPublicKey;
use crate::models::RSARSAPublicKey;
pub type OpaqueData = Vec<u8>;
pub type Ciphertext = Vec<u8>;


pub struct Requester<T> {
    data: T
}
pub struct Replier<T> {
    data: T
}

pub struct Accept<T> {
    data: T
}
pub struct Deny<T> {
    data: T
}

pub enum Establish<T> {
    Accept(T),
    Deny(T),
}


pub enum KeyExchangeProtocol {
    // two parties exchange public keys
    Phase1((Requester<RSAPublicKey>, Replier<RSAPublicKey>)),
    // each party sends ciphertext encrypted with public key exchanged
    // at phase 1, critical information SHALL NOT be exchanged at
    // phase 2
    Phase2((Requester<Ciphertext>, Replier<Ciphertext>)),
    // each party establishes whether phase 2 ciphertext was succesfully decrypted or not (Accept or Deny, respectively).
    // Accept<Ciphertext> payload SHALL bear key material for the next phases:
    // - Time sync information:
    //   - current dateand time in ISO-8601 format
    //   - base32 secret for time-based one-time password
    // - A non-zero number of RSA Public key(s) not yet used or exchanged thus far
    // - A non-zero number of Elliptic Curve Public key(s) not yet used or exchanged thus far
    // - A non-zero number of chacha20 keys
    // - The Rendezvous occasion (space and time) for trusted information exchange
    //   - The Rendezvous space:
    //     - two or more euclidean spaces
    //     - two or more Axes within the geometric boundaries of each euclidean space
    //     - at least one Axis SHALL correspond to any apparent information emerging within the observable space
    //     - exactly one Axis SHALL correspond to the observer of appearances
    //     - each Axis regardless of whether denoting emerging
    //     appearance or reactive observance MAY have its data
    //     (including but not limited to its azimuth and rotation
    //     angles) used as parameters at the occasion of rendezvous,
    //     this SHALL denote key modulation and a sort of uniqueness
    //     of experience of the observer.

    // - A sequential list of key ids used in the encryption of data at the Rendezvous time
    // - Key id must be indistinguishable from one-time password codes
    // Deny<Ciphertext> payload SHALL inform the other party that phase4 is optional
    Phase3((Requester<Establish<Ciphertext>>, Replier<Establish<Ciphertext>>)),
    // Phase4 lasts as long as until the Rendezvous time and SHALL entail:
    // Ciphertext payload SHALL bear key material for the next phases:
    // - Time sync information:
    //   - current dateand time in ISO-8601 format
    //   - base32 secret for time-based one-time password
    // - A non-zero number of RSA Public key(s) not yet used or exchanged thus far
    // - A non-zero number of Elliptic Curve Public key(s) not yet used or exchanged thus far
    // - A non-zero number of chacha20 keys
    // - A sequential list of key ids used in the encryption of data at the Rendezvous time
    // - Key id must be indistinguishable from one-time password codes
    // - Phase 4 MUST be repeated any number of times in order to increase the entropy in the network
    Phase4((Requester<Ciphertext>, Replier<Ciphertext>)),

    // Phase5 validates rendezvous through:
    //
    // 1. time-based one-time password which fails if the time of
    // either party is skewed or somehow adultered.
    //
    // 2. decryption of ciphertext through a pipeline of various
    //    decryption keys as schematically defined in the sequential list
    //    of key ids, that is, the decryption process MUST apply any N
    //    symmetric and assymetric keys in the list of key ids.
    //
    //   2.1. In the case of a key id not denoting valid reference to
    //        decryption key it SHALL denote valid reference to encryption
    //        key.
    //
    //   2.2. In the case of a key id not denoting valid reference to
    //        encryption key it SHALL denote valid reference to Axes
    //        exchanged in phases 3 and 4.
    //
    //   2.3. In the case of a key id denoting Axis:
    //
    //   2.3.1. In the case of the Axis denoting emergent appearance, the opaque data of said appearance MUST be subjected to modulation along with the Axis of its observance, deriving a unique key by means of PBKDF2 algorithm.
    //   2.3.2. In the case of the Axis denoting observance, the opaque data of said observance MAY derive a key using any number of appearances within 2 seconds of the occurrence of its reactions, denoting a collection of unique experiences in space-time.
    //
    // 3. The final plaintext SHALL bear a new set of keys and key
    //    materials establishing trust between the parties including, at
    //    minimum:
    //
    //   3.1. One (reusable) RSA KeyPair (not previously known or used)
    //   3.2. One (reusable) EC KeyPair (not previously known or used)
    //   3.3. One disposable Chacha20 key to encrypt the next message
    //   3.4. One base32 secret for assurance of time integrity at both parties
    Phase5((Requester<Establish<Ciphertext>>, Replier<Establish<Ciphertext>>)),
    // Phase 6: Trust established and critical information MAY be exchanged
    Phase6((Requester<Ciphertext>, Replier<Ciphertext>)),
}

// Inspired by GPG key-signing parties, each user SHALL be represented
// in a "social" graph with edges connecting with participants who
// meet in-person in order to validate each other's actual existence,
// sign each other's public keys and publish the artifact of such
// signature to the key server.

// The trust of each user SHALL effect a kind of gravity in terms of
// the weight of its trust. Actors' trust MUST be either positive or
// negative, and actors MAY revoke trust in other actors at any
// time. The algorithm used in positive trust shall be proportionally
// inverse when applying trust revokation.

// GnuPG inspiration: `gpg --sign-key --ask-cert-level`
// > How carefully have you verified the key you are about to sign actually belongs
// > to the person named above?  If you don't know what to answer, enter "0".
// >
// > (0) I will not answer. (default)
// > (1) I have not checked at all.
// > (2) I have done casual checking.
// > (3) I have done very careful checking.
// Your selection? (enter '?' for more information):

// In the case of the post-quantum public-key infrastructure proposed
// here, more levels of certification SHALL be presented.
//
// The server architecture is a hybrid of federated (centralized) and
// peer validated, that is, users with a determined level of of trust
// (gravity) MAY ellect to run their own validation services having a
// two-fold effect:
// - reconcile checksums with the federation, making it patently clear that the central federation is trustworthy
// - serve as intermediate revokation authority
//
// Non-trusted actors MAY run their own service validating and syncing
// with the federation merely for purposes of allowing the integrity
// of the central federation to be validated and its data available -
// rewards SHALL NOT be given to non-trusted actors nor to actors of
// insufficient trust.
//
// Further reading:
//
// - https://gpgtools.tenderapp.com/kb/faq/what-is-ownertrust-trust-levels-explained
// - https://debconf24.debconf.org/about/ksp/
// - https://www.linuxdays.cz/2018/en/key-signing-party/
//

pub enum TrustLevel {
    Remote(PublicKey),
    InPerson(PublicKey),
}
