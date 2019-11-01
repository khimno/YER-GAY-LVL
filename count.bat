@echo off
SET @var=1
:loop
echo YER GAY LVL: %@var%
SET /a "var=%var%1"
goto loop
