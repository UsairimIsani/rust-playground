struct GenericStruct<T>(T);
struct Container<T> {
    item: T,
}
enum Transmission<T> {
    Signal(T),
    NoSignal,
}
