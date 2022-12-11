
! gfortran titi.f90 -o titi && ./titi

program tata
    
    ! Local variables
    integer, parameter :: read_unit = 99
    integer :: ios
    character(len=15) :: line
    integer, dimension(:),allocatable :: start_1, stop_1, start_2, stop_2
    integer :: nb_lines, i,j
    logical :: cond_1, cond_2
    integer :: answ1, answ2

    ! Read input from file (did not manage to put this in a function...)
    open(unit=read_unit, file="input.txt", iostat=ios)
    if ( ios /= 0 ) stop "Error opening file"
    nb_lines = 0
    do
        read(read_unit, '(A)', iostat=ios) line
        if (ios /= 0) exit
        nb_lines = nb_lines + 1
    end do
    allocate(start_1(nb_lines))
    allocate(stop_1(nb_lines))
    allocate(start_2(nb_lines))
    allocate(stop_2(nb_lines))
    rewind(read_unit)
    do i = 1, nb_lines
        read(read_unit, '(A)') line
        do j = 1,len(line)
            if( line(j:j) == "-" ) then
                line(j:j) = ","
            end if
        end do
        read(line, *) start_1(i), stop_1(i), start_2(i), stop_2(i)
    end do
    close(read_unit)

    do i = 1, nb_lines
        print*, start_1(i), stop_1(i), start_2(i), stop_2(i)
    end do
    
    ! Part 1
    answ1 = 0
    do i = 1, nb_lines
        cond_1 = (start_1(i) >= start_2(i)) .and. (stop_1(i) <= stop_2(i))
        cond_2 = (start_1(i) <= start_2(i)) .and. (stop_1(i) >= stop_2(i))
        if( cond_1 .or. cond_2 ) then
            answ1 = answ1 + 1
        end if
    end do
    print*, "The first answer is", answ1
    
    ! Part 2
    answ2 = 0
    do i = 1, nb_lines
        cond_1 = (start_2(i) <= start_1(i)) .and. (start_1(i) <= stop_2(i))
        cond_2 = (start_1(i) <= start_2(i)) .and. (start_2(i) <= stop_1(i))
        if( cond_1 .or. cond_2 ) then
            answ2 = answ2 + 1
        end if
    end do
    print*, "The second answer is", answ2    
    

end program tata
