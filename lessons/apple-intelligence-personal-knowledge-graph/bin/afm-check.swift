import Foundation
import FoundationModels

@main
struct AFMCheck {
    static func main() {
        guard #available(macOS 26.0, *) else {
            fputs("Foundation Models requires macOS 26 or later.\n", stderr)
            exit(EXIT_FAILURE)
        }

        let model = SystemLanguageModel.default

        switch model.availability {
        case .available:
            print("AFM on-device model: available")
        case .unavailable(.deviceNotEligible):
            print("AFM on-device model: unavailable (device not eligible)")
        case .unavailable(.appleIntelligenceNotEnabled):
            print("AFM on-device model: unavailable (Apple Intelligence is disabled)")
        case .unavailable(.modelNotReady):
            print("AFM on-device model: unavailable (model not ready)")
        @unknown default:
            print("AFM on-device model: unavailable (unknown reason)")
        }
    }
}
