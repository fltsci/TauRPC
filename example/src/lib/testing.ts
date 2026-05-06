import { createTauRPCProxy } from '@fltsci/taurpc'
import { Channel } from '@tauri-apps/api/core'
import type { Router } from './bindings'

const taurpc = createTauRPCProxy<Router>()

const date = new Date()
const bytes = new Uint8Array([1, 2, 3, 4])
const url = new URL('https://specta.dev/docs?example=rich-types')
const channel = new Channel<{ date: Date; bytes: Uint8Array; url: URL }>()

channel.onmessage = (message) => {
  console.log('semanticTypes channel', message)
  console.log(
    'SEMANTIC CHANNEL ASSERTIONS',
    message.date instanceof Date && message.date.getTime() === date.getTime(),
    message.bytes instanceof Uint8Array &&
      message.bytes.length === bytes.length &&
      message.bytes.every((v, i) => v === bytes[i]),
    message.url instanceof URL && message.url.href === url.href,
  )
}

// events.semanticTypesEvent.listen((event) => {
//   console.log('semanticTypesEvent', event.payload)
//   console.log(
//     'SEMANTIC EVENT ASSERTIONS',
//     event.payload.date instanceof Date,
//     event.payload.bytes instanceof Uint8Array,
//     event.payload.url instanceof URL,
//   )
// })

taurpc.semanticTypes({ date, bytes, url }, channel).then((result) => {
  console.log('semanticTypes', result)
  console.log(
    'SEMANTIC TYPE ASSERTIONS',
    result.date.getTime() === date.getTime(),
    result.bytes.length === bytes.length &&
      result.bytes.every((v, i) => v === bytes[i]),
    result.url.href === url.href,
  )

  events.semanticTypesEvent.emit(result)
})
