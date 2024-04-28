module Main

type EcoSystem = FSharp | JS | Rust 

type Message = {
    ecoSystem: EcoSystem
    text: string
}
