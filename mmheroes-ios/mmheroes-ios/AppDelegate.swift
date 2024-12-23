import UIKit
import Version

private let applicationInfoKey = "com.jaskiewiczs.mmheroes.applicationInfoKey"

@main
final class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?

    let applicationInfo = ApplicationInfo(version: Bundle.main.version)

    func application(
        _ application: UIApplication,
        willFinishLaunchingWithOptions
            launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil
    ) -> Bool {
        window = UIWindow()
        return true
    }

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions
            launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        if window?.rootViewController == nil {
            window?.rootViewController = MainSceneViewController()
        }
        window?.makeKeyAndVisible()
        return true
    }

    func application(
        _ application: UIApplication,
        viewControllerWithRestorationIdentifierPath identifierComponents: [String],
        coder: NSCoder
    ) -> UIViewController? {
        if identifierComponents.last == MainSceneViewController.restorationIdentifier {
            let vc = MainSceneViewController()
            window?.rootViewController = vc
            return vc
        }
        return nil
    }

    func application(_ application: UIApplication,
                     shouldSaveSecureApplicationState coder: NSCoder) -> Bool {
        self.application(application, shouldSaveApplicationState: coder)
    }

    func application(_ application: UIApplication,
                     shouldSaveApplicationState coder: NSCoder) -> Bool {
        do {
            try coder.encodeEncodable(applicationInfo, forKey: applicationInfoKey)
            return true
        } catch {
            return false
        }
    }

    func application(_ application: UIApplication,
                     shouldRestoreSecureApplicationState coder: NSCoder) -> Bool {
        self.application(application, shouldRestoreApplicationState: coder)
    }

    func application(_ application: UIApplication,
                     shouldRestoreApplicationState coder: NSCoder) -> Bool {
        do {
            let previousRunApplicationInfo = try coder
                .decodeDecodable(ApplicationInfo.self, forKey: applicationInfoKey)
            return
                previousRunApplicationInfo.version.major == applicationInfo.version.major
        } catch {
            return false
        }
    }
}

