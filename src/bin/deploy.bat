@echo off
REM Deploy script for GitHub Pages

REM Navigate to project root
cd /d "%~dp0\..\.."

echo Deploying to GitHub Pages...

REM Build the project
call src\bin\build.bat
if errorlevel 1 (
    echo Error: Build failed.
    pause
    exit /b 1
)

REM Copy book contents to root for GitHub Pages
echo Copying build output to root...
xcopy /e /i /y "book\*" ".\"

REM Remove book folder since we copied its contents
rmdir /s /q "book"

echo Deployment completed!
echo Files are now ready for GitHub Pages.
echo Commit and push to deploy.
pause
