## Advanced Encryption Standard

This ~~is~~ <b>would be</b> ``Rust`` implementation for ```AES``` based on [FIBS 197](https://csrc.nist.gov/publications/detail/fips/197/final) with three different key sizes, namely: 128 bits, 192 bits, and 256 bits.
</br></br>The ```AES``` algorithm is pretty much state machine with 128 bits/16 bytes state despite the key size, hence the data should be </b>padded</b> to be a multiple of 16 bytes. The recommended padding algorithm is ```PKCS#7``` according to [RFC 2315](https://www.ietf.org/rfc/rfc2315.txt). We are going to have it here.
</br></br>Using block ciphers such as ```AES``` in a naive way is dangerous and can lead to data breaches and malicious tampering with the data. That is why people of encryption needed to devise ways to use block ciphers, hence the need for different modes of operations.</br></br>The supported modes of operations in these repository are:</br>
* Electronic Codebook Mode (ECB)
* Cipher Block Chaining Mode (CBC)
* Output Feedback Mode (OFB)
* Cipher Feedback Mode (CFB)
* Counter Mode (CTR)
* Galois Counter Mode (GCM)

</br></br>Many of these modes are obsolete but I ~~am~~ <b>would be</b> including them for the sake of completeness.

