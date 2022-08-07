# Create an Exchange

An image exchange is the most technically capable may of modifying your image. You can either do them Asyncronously, or Syncronously

## Syncronously

Syncronously is the simplest way to create an exchange, simply serialize your image to Base 64 and send it to the `createExchange` mutation

```gql
mutation CropImage($image: String) {
  createExchange(input: { imageBase64: $image }) {
    base64image
  }
}
```

## Async

Not implemented yet :(
