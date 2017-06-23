require 'os'
require 'fileutils'

target = "./rrake"
tp = "target/release/rrake"
ip = "/usr/bin/rake"

if OS.windows? then
    target = "rrake.exe"
    tp = "target\\release\\rrake.exe"
    ip = "C:\\bin\\rake.exe"
end

task :default do
    sh "cargo build --release"
end

task :test do
    sh "#{target} --help"
    puts
    sh "#{target} --version"
    puts
    sh "#{target} -f Rakefile echo"
    puts
    sh "#{target} -f Rakefile refertoecho"
    puts
    sh "#{target} -f Rakefile pwd"
    puts
    sh "#{target} --rakefile Rakefile countdown"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :install => [:upx] do
    FileUtils.copy(target, ip)
end

task :clean do
    sh "cargo clean"
end

task :cleanlock do
    File.delete("Cargo.lock")
end

task :echo do
    puts "Simple echo task..."
    puts "Prints out a string!"
end

task :sh do
    sh "touch dummy.txt"
end

task :cleansh do
    File.delete("dummy.txt")
end

task :pwd do
    Dir.pwd
end

task :countdown do
    puts "5"
    sleep 1000
    puts "4"
    sleep 1000
    puts "3"
    sleep 1000
    puts "2"
    sleep 1000
    puts "1"
    sleep 1000
    puts "Blast off!"
end

task :refertoecho => [:echo]
