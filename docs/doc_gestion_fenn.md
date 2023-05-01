# Gestion de la fenêtre
## window.create(options)
Cette fonction permet de créer une nouvelle fenêtre pour l'application. Les options suivantes sont disponibles :
- **title** : le titre de la fenêtre.
- **width** : la largeur de la fenêtre.
- **height** : la hauteur de la fenêtre.
- **x** : la position horizontale de la fenêtre sur l'écran.
- **y** : la position verticale de la fenêtre sur l'écran.
- **url** : l'URL du contenu de la fenêtre.
- **iconFilePath** : le chemin d'accès vers l'icône de la fenêtre.

```js
Neutralino.window.create({
  title: "Ma fenêtre",
  width: 800,
  height: 600,
  url: "https://www.example.com",
  iconFilePath: "/path/to/icon.png"
});
```
## window.setTitle(title)
Cette fonction permet de modifier le titre de la fenêtre.

```js
Neutralino.window.setTitle("Nouveau titre");
```

## window.getPosition()
Cette fonction renvoie la position actuelle de la fenêtre sous forme d'objet **\`{x, y}\`**.

```js
const position = Neutralino.window.getPosition();
console.log(`Position: (${position.x}, ${position.y})`);
```

## window.getSize()
Cette fonction renvoie la taille actuelle de la fenêtre sous forme d'objet **\`{width, height}\`**.

```js
const size = Neutralino.window.getSize();
console.log(`Taille: (${size.width}, ${size.height})`);
```

## window.isVisible()
Cette fonction renvoie **\`true\`** si la fenêtre est actuellement visible, **\`false\`** sinon.

```js
const visible = Neutralino.window.isVisible();
console.log(`Fenêtre visible : ${visible}`);
```

## window.show()
Cette fonction affiche la fenêtre si elle n'est pas déjà visible.

```js
Neutralino.window.show();
```

## window.hide()
Cette fonction masque la fenêtre si elle est visible.

```js
Neutralino.window.hide();
```

## window.close()
Cette fonction ferme la fenêtre.

```js
Neutralino.window.close();
```

## window.setFullscreen()
Cette fonction active le mode plein écran pour la fenêtre de l'application.

```js
Neutralino.window.setFullscreen();
```

## window.exitFullScreen()
Cette fonction désactive le mode plein écran pour la fenêtre de l'application.

```js
Neutralino.window.exitFullScreen();
```