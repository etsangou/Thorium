#include "app/app.hpp"

int main() {
    app::application_specification app_spec;
    app_spec.name = "Thorium";
    app_spec.windows_spec.width = 1280;
    app_spec.windows_spec.height = 720;

    // Démarrage de l'application (UI)
    app myApp(app_spec);
    myApp.start();

    return 0;
}