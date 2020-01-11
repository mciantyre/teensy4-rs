@echo off

SET scriptbuildflag=--release
SET scriptbin=%1
IF NOT DEFINED scriptbin (goto :needbin)
IF "%2"=="--debug" (SET "scriptbuildflag=")
IF NOT DEFINED scriptbuildflag (
	SET scriptbuildmode=debug
) ELSE (
	SET scriptbuildmode=release
)
cargo build %scriptbuildflag% -p teensy4-examples --bin %scriptbin% || goto :error
rmdir /S /Q out || goto :error
mkdir out || goto :error
copy "target\thumbv7em-none-eabihf\%scriptbuildmode%\%scriptbin%" "out\%scriptbin%" || goto :error
arm-none-eabi-objdump -d -S -C "out\%scriptbin%" > "out\%scriptbin%.lst" || goto :error
arm-none-eabi-objdump -t -C "out\%scriptbin%" > "out\%scriptbin%.sym" || goto :error
arm-none-eabi-objcopy -O ihex -R .eeprom "out\%scriptbin%" "out\%scriptbin%.hex" || goto :error
echo Now use Teensy Loader to load out\%scriptbin%.hex onto the Teensy 4.
goto :EOF

:needbin
echo Need an example to build, such as 'led'.
exit /b 160

:error
echo Failed with error code %errorlevel%.
exit /b %errorlevel%
