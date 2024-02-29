<?php

$text = "Texte bourés de faute d'othographe.";

var_dump(ispell_check($text, "fr_FR"));