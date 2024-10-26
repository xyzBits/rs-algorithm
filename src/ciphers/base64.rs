/*
   A Rust implementation of a base64 encoder and decoder
   Written from scratch
*/

// The charset and padding used for en- and decoding.
const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PADDING: char = '=';
