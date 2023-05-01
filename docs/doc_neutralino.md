# Doc neutralino
Neutralino.js est un framework léger pour le développement d'applications de bureau multiplateformes

## Fonctions
Les fonctions disponibles dans Neutralino.js incluent :
- **osName()** : renvoie le nom du système d'exploitation actuel (Windows, macOS, Linux)
- **getDistro()** : renvoie le nom de la distribution Linux (si applicable)
- **getFreeDiskSpace()** : renvoie l'espace disque libre sur le disque dur
- **fileSystem** : fournit des méthodes pour accéder au système de fichiers
- **window** : fournit des méthodes pour manipuler la fenêtre de l'application
- **app** : fournit des méthodes pour gérer l'application elle-même, notamment la fermeture de l'application et l'enregistrement de callbacks.

# Démarrage d'une fenêtre
```js
Neutralino.init();

Neutralino.window.create({
  title: "Ma application",
  width: 800,
  height: 600,
  url: "index.html"
});
```
Ce code initialise Neutralino.js et crée une fenêtre de base avec un titre, une taille et une URL de contenu.