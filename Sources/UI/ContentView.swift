import RustLib
import SwiftUI

public struct ContentView: View {
    public init() {}

    public var body: some View {
        VStack {
            Image("Logo", bundle: .resources)
            Text("The answer to life, the universe, and everything is: **\(get_a_value_from_rust())**")
                .multilineTextAlignment(.center)
                .padding(32)
            Button ("Print hello from rust") {
                print_hello_rust()
            }
            Button ("Print string from rust") {
                print(get_a_string_from_rust().toString())
            }
            Button ("Print from rust sidekick") {
                print_from_sidekick()
            }
            Button ("Print Swift string from rust") {
                print_string_from_swift("String from Swift")
            }
            Text("Hey there, \(get_user().nickname.toString())!")
                .padding(32)
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

private class ResourceHandle {}

extension Bundle {
    static let resources = Bundle(for: ResourceHandle.self)
}
