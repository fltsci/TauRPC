import { createTauRPCProxy } from './proxy'

const taurpc = createTauRPCProxy()

const date = new Date()
const bytes = new Uint8Array([1, 2, 3, 4])
const url = new URL('https://specta.dev/docs?example=rich-types')
const input = { date, bytes, url }

const assertSemanticTypes = (
  label: string,
  message: { date: Date; bytes: Uint8Array; url: URL },
) => {
  console.log(label, message)
  console.log(
    `${label} ASSERTIONS`,
    message.date instanceof Date && message.date.getTime() === date.getTime(),
    message.bytes instanceof Uint8Array &&
      message.bytes.length === bytes.length &&
      message.bytes.every((v, i) => v === bytes[i]),
    message.url instanceof URL && message.url.href === url.href,
  )
}

const runSemanticTypesExample = async () => {
  await taurpc.semantic_types_event.on((message) => {
    assertSemanticTypes('semanticTypes event', message)
  })

  const result = await taurpc.semantic_types(input, (message) => {
    assertSemanticTypes('semanticTypes channel', message)
  })

  assertSemanticTypes('semanticTypes result', result)
}

runSemanticTypesExample()
