## cipher-rs

Using `rust` to cipher / decipher cryptograpy

## Quick Glance

Sample outputs with the following

1. Total output

![image](https://i.imgur.com/4ii5g6x.png)

- Frequency analysis of file : `.src/cipher.txt`

```bash
Frequency Analysis :: Ok({27: 129, 72: 88, 15: 81, 57: 81, 54: 75, 69: 73, 66: 68, 39: 59, 48: 51, 21: 44, 36: 39, 51: 35, 75: 34, 24: 25, 60: 21, 30: 15, 87: 15, 78: 14, 33: 10, 18: 9, 63: 4, 81: 4, 84: 3, 90: 2})
```

- Character analysis based on the english language

```bash
Character Analysis :: Ok({27: "e", 72: "t", 15: "a", 57: "o", 54: "n", 69: "i", 66: "h", 39: "s", 48: "r", 21: "l", 36: "d", 51: "u", 75: "c", 24: "m", 60: "w", 30: "y", 87: "f", 78: "g", 33: "p", 18: "b", 63: "v", 81: "k", 84: "j", 90: "x"})
```

- Original file subsituted content (`.src/cipher.txt`)

```bash
File Map :: "a n a t o u s i t d e i u a r r e i t l o n i t s t c e n t c n s t o y o h m s n a h f u a t t e h t d a t l o n i t s t c t e i a l d e u s l a r e r e u e n t e g e h f i o r s m r s v c s m p a i a n m w r a i u a s i l o u w o i e m o y n e c t h a r o h s o n s x e m a t o u i a t o u i a h e e j t h e u e r f i u a r r t f w s l a r i s x e i a h e a h o c n m d c n m h e m w s l o u e t e h i t d e f a h e i o i u a r r t d a t a l l c h a t e r f w h e m s l t s n p t d e s h b e d a g s o h c i s n p l r a i i s l a r w d f i s l i a i s y t d e f k e h e b s r r s a h m b a r r i y o h e j a u w r e s i n o t w o i i s b r e t d s i s i m c e t o v c a n t c u e y y e l t i l c h h e n t a t o u s l u o m e r i n o k c i e v c a n t c u w h s n l s w r e i t o b e t t e h e j w r a s n a n m w h e m s l t t d s i b e d a g s o h e g e h f a t o u s i l o u w o i e m o y a n c l r e c i a n m o n e o h u o h e e r e l t h o n i b o c n m t o t d e n c l r e c i t d e n c l r e c i s i u a m e o y o n e o h u o h e w h o t o n i a n m a n c u b e h o y n e c t h o n i o n r f t d e u o i t l o u u o n g a h s e t f o y d f m h o p e n d a i n o n e c t h o n i w h o t o n i a n m n e c t h o n i a h e l a r r e m n c l r e o n i u o h e t d a n n s n e t f n s n e o y a n a t o u i u a i i s i s n t d e n c l r e c i t d e w h o t o n i d a g e a w o i s t s g e e r e l t h s l l d a h p e k d e h e a i t d e e r e l t h o n i d a g e a n e p a t s g e e r e l t h s l l d a h p e t d e n e c t h o n i d a g e n o e r e l t h s l l d a h p e s y t d e n c u b e h o y w h o t o n i a n m e r e l t h o n i a h e e v c a r t d e n t d e a t o u s i e r e l t h s l a r r f n e c t h a r s y a n a t o u d a i u o h e o h y e k e h e r e l t h o n i t d a n w h o t o n i t d e n s t d a i a n o g e h a r r n e p a t s g e o h w o i s t s g e l d a h p e h e i w e l t s g e r f t d e i e a t o u i a h e l a r r e m s o n i "
```

- Modified substitution (not sent to any file) `WIP`

```bash
File Mod :: "a n a t o m i s t h e s m a l l e s t c o n s t i t u e n t u n i t o f o r d i n a r y m a t t e r t h a t c o n s t i t u t e s a c h e m i c a l e l e m e n t e v e r y s o l i d l i q u i d g a s a n d p l a s m a i s c o m p o s e d o f n e u t r a l o r i o n i z e d a t o m s a t o m s a r e e x t r e m e l y s m a l l t y p i c a l s i z e s a r e a r o u n d h u n d r e d p i c o m e t e r s t h e y a r e s o s m a l l t h a t a c c u r a t e l y p r e d i c t i n g t h e i r b e h a v i o r u s i n g c l a s s i c a l p h y s i c s a s i f t h e y w e r e b i l l i a r d b a l l s f o r e x a m p l e i s n o t p o s s i b l e t h i s i s d u e t o q u a n t u m e f f e c t s c u r r e n t a t o m i c m o d e l s n o w u s e q u a n t u m p r i n c i p l e s t o b e t t e r e x p l a i n a n d p r e d i c t t h i s b e h a v i o r e v e r y a t o m i s c o m p o s e d o f a n u c l e u s a n d o n e o r m o r e e l e c t r o n s b o u n d t o t h e n u c l e u s t h e n u c l e u s i s m a d e o f o n e o r m o r e p r o t o n s a n d a n u m b e r o f n e u t r o n s o n l y t h e m o s t c o m m o n v a r i e t y o f h y d r o g e n h a s n o n e u t r o n s p r o t o n s a n d n e u t r o n s a r e c a l l e d n u c l e o n s m o r e t h a n n i n e t y n i n e o f a n a t o m s m a s s i s i n t h e n u c l e u s t h e p r o t o n s h a v e a p o s i t i v e e l e c t r i c c h a r g e w h e r e a s t h e e l e c t r o n s h a v e a n e g a t i v e e l e c t r i c c h a r g e t h e n e u t r o n s h a v e n o e l e c t r i c c h a r g e i f t h e n u m b e r o f p r o t o n s a n d e l e c t r o n s a r e e q u a l t h e n t h e a t o m i s e l e c t r i c a l l y n e u t r a l i f a n a t o m h a s m o r e o r f e w e r e l e c t r o n s t h a n p r o t o n s t h e n i t h a s a n o v e r a l l n e g a t i v e o r p o s i t i v e c h a r g e r e s p e c t i v e l y t h e s e a t o m s a r e c a l l e d i o n s "
```

The proper text can be read as:

anatomis the smallest constituent unit of ordinary matter that constitutes a chemical element every solid liquid gas and plasma is composed of neutral or
ionized atoms atoms are extremely small typical sizes are around hundred pico meters they are so small that accurately predicting their behavior using classical
physics as if they were billiard balls for example is not possible this is due to quantum effects current atomic models now use quantum principles to better explain and
predict this behavior every atom is composed of a nucleus and one or more electrons bound to the nucleus the nucleus is made of one or more protons and a number of neutrons
only the most common variety of hydrogen has no neutrons protons and neutrons are called nucleons more than ninety nine of an atom s mass is in the nucleus the protons have a
positive electric charge whereas the electrons have a negative electric charge the neutrons have no electric charge if the number of protons and electrons are equal then the atom is
electrically neutral if an atom has more or fewer electrons than protons then it has an overall negative or positive charge respectively these atoms are called ions
