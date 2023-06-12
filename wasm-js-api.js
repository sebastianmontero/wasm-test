export function loggear(str) {
  console.log(str)
}

export function syncfn(str) {
  return "paso por js: " + str
}

export async function asyncfn2(str) {
  const p = new Promise(resolve => setTimeout(resolve("paso por js2: " + str), 1000))
  return p
}

export async function asyncfne(str) {
  const p = new Promise((resolve, reject) => setTimeout(reject("error: " + str), 1000))
  return p
}

export async function asyncfn(str) {
  const p = new Promise(resolve => setTimeout(resolve("paso por js1: " + str), 1000))
  return p
}

export function is_nostr_available () {
  return !!window.nostr
}

export async function enable () {
  return window.nostr.enable()
}

export async function get_public_key () {
  return window.nostr.getPublicKey()
}

export async function encrypt (pubkey, plaintext) {
  return window.nostr.nip04.encrypt(pubkey, plaintext)
}

export async function decrypt (pubkey, ciphertext) {
  return window.nostr.nip04.decrypt(pubkey, ciphertext)
}

export async function sign_event (event) {
  let {sig} = await window.nostr.signEvent(event)
  return sig
}


// export async function asyncfn(str) {
//   return "paso por js1: " + str
// }