# Allocateur Slab & SLUB Linux - Documentation Technique

## Auteurs
- BOUAZRA Mehdi
- MALIH Omar

---

## Introduction

Ce document explique en détail ce qu'est un allocateur slab, pourquoi il est utilisé dans les noyaux de systèmes d'exploitation, et comment fonctionne l'allocateur SLUB actuellement utilisé dans le noyau Linux. Nous présentons également notre propre implémentation simplifiée.

---

## Partie 1 : Le Problème de l'Allocation Mémoire dans un Kernel

### Pourquoi l'allocation mémoire est critique

Dans un système d'exploitation, le noyau doit constamment allouer et libérer de la mémoire pour diverses structures : descripteurs de processus, buffers réseau, inodes du système de fichiers, etc. Ces allocations ont des caractéristiques particulières :

1. Les objets sont souvent de petite taille (quelques dizaines à quelques centaines d'octets)
2. Les allocations et désallocations sont très fréquentes
3. Beaucoup d'objets ont la même taille
4. La performance est critique car le kernel gère tout le système

### Les problèmes des allocateurs traditionnels

Un allocateur classique comme malloc pose plusieurs problèmes dans ce contexte :

**Fragmentation externe** : Après de nombreuses allocations et libérations, la mémoire libre devient dispersée en petits morceaux. Même si la somme totale de mémoire libre est suffisante, il devient impossible d'allouer un bloc contigu de la taille demandée.

**Fragmentation interne** : Les allocateurs arrondissent souvent la taille demandée à une taille standard, gaspillant de la mémoire.

**Lenteur** : À chaque allocation, l'allocateur doit parcourir une structure de données pour trouver un bloc libre de taille appropriée. Cette recherche prend du temps.

**Mauvaise utilisation du cache CPU** : Les objets du même type se retrouvent dispersés en mémoire, ce qui rend le cache processeur moins efficace.

---

## Partie 2 : La Solution - L'Allocateur Slab

### Concept fondamental

L'allocateur slab a été inventé par Jeff Bonwick chez Sun Microsystems en 1994 pour le système Solaris. L'idée est simple mais puissante : au lieu d'allouer la mémoire de façon générique, on crée des "caches" spécialisés pour chaque type d'objet.

Un **slab** est une région de mémoire contiguë (généralement une ou plusieurs pages) divisée en objets de taille identique. Tous les objets d'un slab ont exactement la même taille.

### Fonctionnement détaillé

**Initialisation** : Quand on crée un slab, on divise la mémoire en N objets de taille fixe. Tous ces objets sont ajoutés à une liste libre (free list).

**Allocation** : Pour allouer un objet, on retire simplement le premier élément de la liste libre. C'est une opération en O(1), extrêmement rapide.

**Libération** : Pour libérer un objet, on l'ajoute au début de la liste libre. C'est également une opération en O(1).

### La liste libre intrusive

Une astuce importante est que la liste libre est "intrusive" : au lieu de maintenir une structure de données séparée pour suivre les blocs libres, on utilise les blocs libres eux-mêmes pour stocker les pointeurs de la liste. Chaque bloc libre contient à son début un pointeur vers le prochain bloc libre.

Cette technique élimine le besoin de métadonnées externes et économise de la mémoire.

### Avantages de l'approche slab

**Performance** : Allocation et désallocation en temps constant O(1).

**Pas de fragmentation** : Tous les objets ont la même taille, donc pas de fragmentation externe. Et comme on alloue exactement la taille nécessaire, pas de fragmentation interne non plus.

**Bonne utilisation du cache** : Les objets du même type sont regroupés en mémoire, ce qui améliore la localité spatiale et l'efficacité du cache CPU.

**Réutilisation des objets** : Un objet libéré reste "chaud" dans le cache et peut être rapidement réalloué.

---

## Partie 3 : Les Allocateurs du Noyau Linux

### Historique

Le noyau Linux a connu trois allocateurs slab au fil du temps :

**SLAB (1996-2007)** : Premier allocateur slab de Linux, porté depuis Solaris. Il était fonctionnel mais complexe, avec environ 10 000 lignes de code. Il utilisait plusieurs niveaux de caches par CPU et des files d'attente complexes.

**SLUB (2007-présent)** : Remplacement simplifié de SLAB, devenu l'allocateur par défaut. Environ 5 000 lignes de code. Il supprime beaucoup de la complexité de SLAB tout en gardant de bonnes performances.

**SLOB** : Allocateur minimal pour les systèmes embarqués avec très peu de mémoire. Il sacrifie les performances pour minimiser l'empreinte mémoire.

### Pourquoi SLUB a remplacé SLAB

SLAB avait plusieurs défauts que SLUB corrige :

**Complexité excessive** : SLAB maintenait plusieurs files d'attente par CPU, des listes de slabs pleins, partiels et vides, et beaucoup de métadonnées. Cette complexité rendait le code difficile à maintenir et à déboguer.

**Surcharge mémoire** : Les nombreuses structures de données de SLAB consommaient de la mémoire.

**Problèmes de scalabilité** : Sur les systèmes avec beaucoup de CPUs, les verrous de SLAB devenaient un goulot d'étranglement.

SLUB simplifie tout cela en utilisant une seule liste libre par slab et en stockant les métadonnées directement dans la structure page du kernel.

---

## Partie 4 : Architecture de SLUB

### Les kmem_cache

Dans SLUB, chaque taille d'objet a son propre "cache" appelé kmem_cache. Par exemple, il existe kmalloc-32 pour les objets de 32 octets, kmalloc-64 pour 64 octets, etc.

Chaque kmem_cache contient :
- Un pointeur vers le slab actif pour chaque CPU
- La taille des objets
- Le nom du cache (pour le débogage)
- Des statistiques d'utilisation

### Slabs par CPU

Pour éviter les contentions de verrou, chaque CPU a son propre slab actif. Quand un CPU veut allouer un objet, il utilise son slab local sans avoir besoin de verrou. C'est le "fast path" qui est extrêmement rapide.

Si le slab local est épuisé, le CPU doit acquérir un verrou pour obtenir un nouveau slab depuis une liste partagée. C'est le "slow path", plus lent mais rare.

### La liste des slabs partiels

SLUB maintient une liste des slabs qui ne sont ni pleins ni vides. Quand un CPU a besoin d'un nouveau slab, il prend d'abord dans cette liste pour réutiliser un slab partiellement rempli avant d'en allouer un nouveau.

### Stockage de la freelist

SLUB stocke le pointeur vers le premier objet libre directement dans la structure page du kernel. Chaque objet libre contient un pointeur vers le prochain objet libre, formant une liste chaînée.

### Chemin d'allocation typique

Quand le kernel appelle kmalloc avec une certaine taille :

1. SLUB détermine quel kmem_cache utiliser selon la taille demandée
2. Il regarde le slab actif du CPU courant
3. Si la freelist n'est pas vide, il retire le premier objet (fast path, pas de verrou)
4. Si la freelist est vide, il cherche un slab dans la liste partielle (slow path, avec verrou)
5. Si aucun slab partiel n'existe, il alloue un nouveau slab
6. Il retourne le pointeur vers l'objet alloué

### Chemin de libération

Quand le kernel libère un objet avec kfree :

1. SLUB détermine à quel slab appartient l'objet (via la structure page)
2. Il ajoute l'objet au début de la freelist du slab
3. Si c'était un slab plein, il devient partiel et est ajouté à la liste partielle
4. Si le slab devient complètement vide, il peut être libéré vers le système

---

## Partie 5 : Fonctionnalités de débogage de SLUB

SLUB inclut des outils puissants pour détecter les bugs mémoire :

**Red Zoning** : SLUB peut ajouter des zones de garde avant et après chaque objet. Si ces zones sont modifiées, c'est qu'il y a eu un dépassement de buffer.

**Poisoning** : Quand un objet est libéré, SLUB peut le remplir avec un motif spécial. Si ce motif est modifié avant la prochaine allocation, c'est qu'il y a eu une utilisation après libération (use-after-free).

**User Tracking** : SLUB peut enregistrer où chaque allocation a été faite, facilitant le débogage.

Ces fonctionnalités s'activent au démarrage du kernel avec le paramètre slub_debug.

---

## Partie 6 : Notre Implémentation

### Objectifs

Notre implémentation est volontairement simplifiée pour des raisons pédagogiques. Elle démontre les concepts fondamentaux de l'allocateur slab sans la complexité d'une implémentation de production.

### Architecture

Notre SlabAllocator maintient trois slabs de tailles différentes :
- slab_32 : pour les objets jusqu'à 32 octets
- slab_64 : pour les objets de 33 à 64 octets
- slab_128 : pour les objets de 65 à 128 octets

Chaque slab est protégé par un Mutex (spin lock) pour la thread safety.

### Structure Slab

Chaque Slab contient :
- object_size : la taille des objets
- free_list : la liste chaînée des objets libres
- base : l'adresse de début de la mémoire
- capacity : le nombre total d'objets

### Structure FreeList

La FreeList est une liste chaînée intrusive. Elle contient un pointeur head vers le premier bloc libre. Chaque bloc libre contient un pointeur vers le suivant.

### Algorithme d'allocation

1. Déterminer le slab approprié selon la taille demandée
2. Verrouiller le slab
3. Retirer un objet de la freelist (pop)
4. Déverrouiller et retourner le pointeur

Si la freelist est vide, on retourne null.

### Algorithme de libération

1. Déterminer le slab approprié selon la taille
2. Verrouiller le slab
3. Ajouter l'objet à la freelist (push)
4. Déverrouiller

### Différences avec SLUB

Notre implémentation diffère de SLUB sur plusieurs points :

**Pas de slabs par CPU** : Nous utilisons un seul slab partagé par taille avec un verrou. SLUB utilise des slabs par CPU pour éviter les verrous sur le chemin rapide.

**Pas de liste partielle** : Nous n'avons pas de mécanisme pour gérer plusieurs slabs de la même taille. Un slab épuisé reste épuisé.

**Moins de tailles** : Nous n'avons que 3 tailles alors que SLUB en a beaucoup plus pour minimiser le gaspillage.

**Pas de débogage** : Nous n'avons pas les fonctionnalités de red zoning ou poisoning.

Ces simplifications rendent notre code plus facile à comprendre tout en illustrant les concepts clés.

---

## Partie 7 : Implications pour la Sécurité

### Pourquoi comprendre les allocateurs slab

La compréhension des allocateurs slab est essentielle pour la sécurité offensive et défensive :

**Exploitation de vulnérabilités** : De nombreuses vulnérabilités kernel impliquent la mémoire heap. Comprendre comment fonctionne l'allocateur permet de transformer un bug en exploitation.

**Use-After-Free (UAF)** : Quand un objet est libéré puis réalloué comme un type différent, un attaquant peut manipuler des pointeurs de fonction ou des données sensibles.

**Heap Overflow** : Un dépassement de buffer peut écraser des objets adjacents dans le même slab.

**Heap Spraying** : Technique pour placer des données contrôlées à des adresses prévisibles en remplissant le heap.

### Protections modernes

Les kernels modernes incluent des protections :

**Randomisation de la freelist** : L'ordre des objets dans la freelist est randomisé pour rendre les exploits moins fiables.

**Hardened usercopy** : Vérifications supplémentaires lors des copies entre kernel et userspace.

**SLAB_FREELIST_HARDENED** : Protection contre la corruption de la freelist.

---

## Conclusion

L'allocateur slab est une technique fondamentale pour l'allocation mémoire efficace dans les systèmes d'exploitation. Son principe est simple mais son impact sur les performances est significatif.

SLUB, l'allocateur actuel de Linux, représente un bon équilibre entre performance et simplicité. Notre implémentation, bien que simplifiée, démontre les concepts clés : la division en objets de taille fixe, la liste libre intrusive, et l'allocation en temps constant.

Cette compréhension est essentielle non seulement pour le développement système mais aussi pour la sécurité informatique, car de nombreuses vulnérabilités exploitent les mécanismes d'allocation mémoire.

---

## Références

1. Bonwick, Jeff. "The Slab Allocator: An Object-Caching Kernel Memory Allocator". USENIX Summer 1994 Technical Conference, 1994.

2. Corbet, Jonathan. "The SLUB allocator". LWN.net, 2007. https://lwn.net/Articles/229984/

3. Opp, Philipp. "Writing an OS in Rust". https://os.phil-opp.com/

4. Love, Robert. "Linux Kernel Development", 3rd Edition. Addison-Wesley, 2010.

5. Code source du noyau Linux : mm/slub.c

6. Documentation kernel : https://www.kernel.org/doc/html/latest/mm/slub.html
