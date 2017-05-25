require 'os'

target = "./rrake"
tp = "target/release/rrake"

if OS.windows? then
    target = "rrake.exe"
    tp = "target\\release\\rrake.exe"
end

task :default do
    sh "cargo build --release"
end

task :test do
    sh "#{target} --help"
    puts
    sh "#{target} --version"
    puts
    sh "#{target} echo"
end

task :cleanupx do
    File.delete(target)
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean do
    sh "cargo clean"
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
