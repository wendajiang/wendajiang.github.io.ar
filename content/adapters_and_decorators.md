---
title: Adapters and Decorators
description: 'adapters and decorator pattern'
template: blog/page.html
date: 2023-03-20 13:18:47
updated: 2023-03-20 13:18:47
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["design pattern"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

# the decorator pattern

> structural pattern, it allows a behavior to be added to an object
>
> The decorator implements the interface of the original class and forwards the requests from its own interface to that class, but it also performs additional actions before and after these forwarded requests -- these are the *decorations*.

```cpp
// original class
class Unit {
  public:
    Unit(double strength, double armor): strength_(strength), armor_(armor) {}
    virtual bool hit(Unit& target) { return attack() > target.defense(); }
    virtual double attack() = 0;
    virtual double defense() = 0;
  protected:
    double strength_;
    double armor_;
};

// concrete class of Unit
class Knight: public Unit {
  public:
    using Unit::Unit;
    double attack() { return strength_ + sword_bonus_; }
    double defense() { return armor_ + plate_bonus_; }
  protected:
    static constexpr double sword_bonus_ = 2;
    static constexpr double plate_bonus_ = 3;
};
class Ogre: public Unit {
  public:
    double attack() { return strength_ + club_penalty_; }     
    double defense() { return armor_ + leather_penalty_; }
  protected:
    static constexpr double club_penalty_ = 2;
    static constexpr double leather_penalty_ = 3; 
};


// use 
Knight k(10, 5);
Ogre o(12, 2);
k.hit(0); // yes!
```
Here the knight, aided by his attack bonus and the enemy's weak armor, will successfully hit the ogre. But the game is far from over. As the units fight, the surviving ones gain experience and eventually become veterans. A veteran unit is still the same kind of unit, but it gains attack and defense bonuses, reflecting its combat experience.

We do not want to change any of the class interfaces, but we want to modify the behavior of the `attack()` and `defense()` functions. This is the job of the decorator pattern, and what follows is the classic implementation of the VeteranUnit decorator:
```cpp
class VeteranUnit : public Unit {     
  public:     
    VeteranUnit(Unit& unit, double strength_bonus, double armor_bonus) :
        Unit(strength_bonus, armor_bonus), unit_(unit) {}     
    double attack() { return unit_.attack() + strength_; }     
    double defense() { return unit_.defense() + armor_; }     
  private:     
    Unit& unit_; 
};

// use
Knight k(10, 5);
Ogre o(12, 2);
VeteranUnit vk(k, 7, 2);
VeteranUnit vo(o, 1, 9);
vk.hit(vo); // another hit!

VeteranUnit vvo(vo, 1, 9);
vk.hit(voo); // miss
// we can decorate a decorated object in this design!
```

## the classic decorator pattern limitation
- the lifetimes of these objects must be carefully managed
- to C++
  - when designer have added a special ability to the Knight unit, but the VeteranUnit decorator do not know the added ability
    can't handle cross-casting well(casting to a type in another branch of the same hierarchy)

## Decorators the C++ way
To solve the two problem, we indorduct the CRTP C++ idiom

```cpp
template <typename U> 
class VeteranUnit : public U {     
  public:     
    VeteranUnit(U&& unit, double strength_bonus, double armor_bonus) :         
      U(unit), strength_bonus_(strength_bonus), armor_bonus_(armor_bonus)     {}     
    double attack() { return U::attack() + strength_bonus_; }     
    double defense() { return U::defense() + armor_bonus_; }     
  private:     
    double strength_bonus_;     
    double armor_bonus_; 
};

// using
Knight k(10, 5); 
Ogre o(12, 2); 
k.hit(o);        // Hit! 
VeteranUnit<Knight> vk(std::move(k), 7, 2); 
VeteranUnit<Ogre> vo(std::move(o), 1, 9); vk.hit(vo);      // Hit! 
VeteranUnit<VeteranUnit<Ogre>> vvo(std::move(vo), 1, 9); 
vk.hit(vvo);     // Miss...
vk.charge();     // Compiles now, vk is a Knight too , this is the Knight added ability, and now is ok
vk.hit(vvo);     // Hit with the charge bonus!
```

This preservation of the interface is a fundamental feature of the Decorator pattern. It is also one of its most serious limitations.

# adapter pattern
It is a sturctural pattern that allows an interface of a class to be used as another, different interface. It allows an existing class to be used in code that expects a different interface, without modifying the original class.

Adapter is very general, broad pattern. It can be used to implement several other, more narrowly defined patterns -- in particular, the decorator.

Converting an object from its current interface to the interface needed by a particular application, without rewriting the object itself, is the purpose and the use of the Adapter pattern.

- class adapter
- function adapter (std::bind)

## compile-time adapters
