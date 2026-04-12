#include "app/app.hpp"

int main() {
    // Création de la fenêtre
    if (!glfwInit()) return 1;
    GLFWwindow* window = glfwCreateWindow(1280, 720, "Thorium", NULL, NULL);
    glfwMakeContextCurrent(window);

    // Démarrage de l'application (UI)
    app myApp(window);
    myApp.start();

    return 0;
}