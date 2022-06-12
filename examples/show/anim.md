# Animations!

<!-- slide -->

## Static CSS Animations!

<svg height="200" width="750" class="static">
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
</svg>

<div class="fragment"><div>
<div class="fragment"><div>

<style>
svg circle {
    transition: all 2s ease-in-out;
}
.present[data-fragment="0"] .static circle {
    fill: blue;
}
.present[data-fragment="1"] .static circle {
    fill: blue;
    transform: translate(650px, 0);
}
</style>

<!-- slide -->

## Keyframe CSS Animations!

<svg height="200" width="750" class="dynamic">
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
</svg>

<div class="fragment"><div>

<style>
@keyframes keymove  {
   0%   { transform:scale(0.5); opacity:0.0; transform:translate(0, 0)}
    50%  { transform:scale(1.2); opacity:0.5;}
    100% { transform:scale(1.0); opacity:1.0; transform:translate(650px, 0); fill: blue}
}
.present .dynamic circle {
    display: none;
}
.present[data-fragment="0"] .dynamic circle {
    display: block;
    animation: keymove 2s ease-in-out forwards;
}
</style>
