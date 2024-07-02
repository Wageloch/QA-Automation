<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Wageloch - Edit Timesheet             _0cb0f5</name>
   <tag></tag>
   <elementGuidId>194b17ab-f85d-47f9-99d8-ac6e0c58c6db</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html.js.flexbox.flexboxlegacy.canvas.canvastext.webgl.no-touch.geolocation.postmessage.no-websqldatabase.indexeddb.hashchange.history.draganddrop.websockets.rgba.hsla.multiplebgs.backgroundsize.borderimage.borderradius.boxshadow.textshadow.opacity.cssanimations.csscolumns.cssgradients.cssreflections.csstransforms.csstransforms3d.csstransitions.fontface.generatedcontent.video.audio.localstorage.sessionstorage.webworkers.no-applicationcache.svg.inlinesvg.smil.svgclippaths</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>html</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>03b4d42a-9f21-4dec-9887-246ed44ddf5f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
      <webElementGuid>4f9a01d2-7c54-402f-be4b-bf551b4ee78f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value> js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths</value>
      <webElementGuid>598f2b20-deed-47a6-bc8e-e4f659d79629</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Wageloch - Edit Timesheet






        
        























    
    
    
        let webAppPreSubmit = (function () {
            let scrollPos = 0;
            function init() {
                setupPayItemsToggle();
                const btnSearch = document.getElementById('btnSearch');
                if (btnSearch != null) btnSearch.addEventListener('click', staffRecordScrollIntoView, true);
                const preSubmitList = document.getElementById('preSubmitList');
                if (preSubmitList != null) preSubmitList.addEventListener('scroll', getScrollPos, true);
                $('[data-wl-payitemhrs]').on('keyup', function (e) { validateHrs(e); });
                setScrollPos();
            }
            function setupPayItemsToggle() {
                const d = document.querySelectorAll('[data-wl-payitemheader]');
                for (let i = 0, l = d.length; i &lt; l; i++) {
                    const ele = d[i];
                    const attr = ele.getAttribute('data-wl-payitemheader');
                    ele.addEventListener('click', function () { togglePayItem(this, attr); }, false);
                }
            }
            function togglePayItem(ele, attr) {
                const t = ele.getAttribute('data-wl-toggle');
                const header = attr == 'allpayitems' ? '[data-wl-payitemheader]' : '[data-wl-payitemheader=&quot;' + attr + '&quot;]';
                const selector = attr == 'allpayitems' ? '[data-wl-payitems]' : '[data-wl-payitems=&quot;' + attr + '&quot;]';
                const i = attr == 'allpayitems' ? '[data-wl-togglei]' : '[data-wl-togglei=&quot;' + attr + '&quot;]';
                if (t == '1') {
                    $(header).attr('data-wl-toggle', 0);
                    $(i).removeClass('fa-caret-down').addClass('fa-caret-up');
                    $(selector).slideDown();
                } else {
                    $(header).attr('data-wl-toggle', 1);
                    $(i).addClass('fa-caret-down').removeClass('fa-caret-up');
                    $(selector).slideUp();
                }
            }
            function staffRecordScrollIntoView() {
                let lowerCaseValue = '';
                const t = document.querySelector('[data-wl-searchinput=&quot;staff&quot;]');
                if (t != null) lowerCaseValue = t.value.toLowerCase();
                const emps = document.querySelectorAll('[data-wl-empname]');
                for (let i = 0, l = emps.length; i &lt; l; i++) {
                    const emp = emps[i];
                    const name = emp.getAttribute('data-wl-empname');
                    if (name.toLowerCase().includes(lowerCaseValue)) {
                        emp.scrollIntoView();
                        break;
                    }
                }
            }
            function getScrollPos() {
                const preSubmitList = document.getElementById('preSubmitList');
                if (preSubmitList != null) scrollPos = preSubmitList.scrollTop;
            }
            function setScrollPos() {
                const preSubmitList = document.getElementById('preSubmitList');
                if (preSubmitList != null) preSubmitList.scrollTop = scrollPos;
            }
            function validateHrs(e) {
                const t = e.target;
                const v = t.value;
                if (v &lt; 0) t.value = 0;
            }
            return {
                init: init
            }
        }());
    



    
        /*.badge-notify {
            background: red;
            position: relative !important;
            top: -12px;
            left: -15px;
            font-size-adjust: 0.4;
        }*/

        #xCentralisedSiteSelection {
            max-width: 250px;
            float: right;
            margin-right: 20px;
            z-index: 9999;
            position: absolute;
            top: 100px;
            right: 0;
        }
        /*the container must be positioned relative:*/
        .custom-select {
            position: relative;
            font-family: Arial;
        }

            .custom-select select {
                display: none; /*hide original SELECT element:*/
            }

        .select-selected {
            background-color: DodgerBlue;
        }
            /*style the arrow inside the select element:*/
            .select-selected:after {
                position: absolute;
                content: &quot;&quot;;
                top: 14px;
                right: 10px;
                width: 0;
                height: 0;
                border: 6px solid transparent;
                border-color: #fff transparent transparent transparent;
            }
            /*point the arrow upwards when the select box is open (active):*/
            .select-selected.select-arrow-active:after {
                border-color: transparent transparent #fff transparent;
                top: 7px;
            }
        /*style the items (options), including the selected item:*/
        .select-items div, .select-selected {
            color: #ffffff;
            padding: 8px 16px;
            border: 1px solid transparent;
            border-color: transparent transparent rgba(0, 0, 0, 0.1) transparent;
            cursor: pointer;
        }
        /*style items (options):*/
        .select-items {
            position: absolute;
            background-color: DodgerBlue;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 99;
        }
        /*hide the items when the select box is closed:*/
        .select-hide {
            display: none;
        }

        .select-items div:hover, .same-as-selected {
            background-color: rgba(0, 0, 0, 0.1);
        }
    
    
        function showhelp() {
            myWindow = window.open(&quot;/ShowHelp&quot;, &quot;_help&quot;, &quot;width=800, height=600&quot;);
        }
        let webAppMaster = function () {
            function registerBtnClocktimesEvent(btnID, sLink) {
                $('#' + btnID).click(function (e) {
                    window.open(sLink, &quot;ClockTimes&quot;);
                    this.blur();
                    return false;
                });
            }
            function registerHR(btnID, sLink) {
                $('#' + btnID).click(function (e) {
                    window.open(sLink, &quot;HR&quot;);
                    this.blur();
                    return false;
                });
            }
            function registerLogoff(btnID, changeID) {
                $('#' + btnID).click(function (e) {
                    const c = document.getElementById(changeID);
                    if (c != null) {
                        const changed = webAppUtils.parseBool(c.value);
                        if (changed) {
                            webAppUtils.msgShow('Sign Out', 'Changes that you made will not be saved.&lt;br>&lt;br>Do you still want to continue?', webAppConfig.MsgBoxStyle.YesNo, webAppSession.logoutAllPages);
                        } else
                            webAppSession.logoutAllPages();
                    }
                    return false;
                });
                const a = document.getElementById('chkAAAMode');
                if (a != null) a.addEventListener('change', function () { webAppUtils.aaaModeAPI(this.checked); }, false);
                const d = document.getElementById('chkDarkMode');
                if (d != null) d.addEventListener('change', function () { webAppUtils.darkModeAPI(this.checked); }, false);
            }
            return {
                registerBtnClocktimesEvent: registerBtnClocktimesEvent,
                registerHR: registerHR,
                registerLogoff: registerLogoff
            }
        }();
    
    
        window.embeddedChatbotConfig = {
            chatbotId: &quot;DK_sXVyG_1_XSidG5zaMQ&quot;,
            domain: &quot;www.chatbase.co&quot;
        }
    
    
    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

    
    









//&lt;![CDATA[
var theForm = document.forms['frmWebApp'];
if (!theForm) {
    theForm = document.frmWebApp;
}
function __doPostBack(eventTarget, eventArgument) {
    if (!theForm.onsubmit || (theForm.onsubmit() != false)) {
        theForm.__EVENTTARGET.value = eventTarget;
        theForm.__EVENTARGUMENT.value = eventArgument;
        theForm.submit();
    }
}
//]]&gt;







//&lt;![CDATA[
if (typeof(Sys) === 'undefined') throw new Error('ASP.NET Ajax client-side framework failed to load.');
//]]&gt;






//&lt;![CDATA[
function WebForm_OnSubmit() {
if (webAppSession.sessionExpired()) return false;webAppUtils.removeElement(webAppConfig.ID.ROSTERSTRIP_TABLE_BODY);$('#MainContent_ucEditTimesheet_expEditTimesheet').wijSaveState('c1expander');$('#ctl00_ctl17').wijSaveState('c1gridview', 'onwijstatesaving');
return true;
}
//]]&gt;




	
	

        
        
        
        
        
//&lt;![CDATA[
Sys.WebForms.PageRequestManager._initialize('ctl00$ScriptManager1', 'frmWebApp', ['tctl00$UpdatePanelMain','UpdatePanelMain'], [], [], 600, 'ctl00');
//]]&gt;


        
                
                
                
                
                    
                        
                            
                                
                                
                                
                            
                            
                        
                        
                            
                                
                                    Dashboard
                                
                                
                                    
                                
                                
                                    Rosters
                                
                                
                                    Timesheets
                                
                                
                                    Staff
                                
                                
                                    Clock Times
                                
                                
                                    Reports
                                
                                
                                    HR
                                
                            
                            
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                            
                                        
                                    
                                
                                
                                    
                                        
                                        
                                    
                                    
                                        
                                        General
                                            
                                        
                                        Roster
                                        
                                        Timesheet
                                        
                                        
                                        
                                        
                                            Portal/Mobile App
                                           
                                            
                                            
                                                Leave
                                                Bulk Mobile Staff Clock
                                                Mobile Proximity Settings
                                                Portal Setup
                                            
                                        
                                        
                                        
                                            Payroll
                                        
                                            
                                            
                                                Pay Level
                                                Payroll Connection
                                                
                                                Payroll Mappings
                                                    
                                                
                                                
                                            
                                        
                                        
                                        
                                            Staff
                                                
                                            
                                            
                                                
                                                    Import
                                                        
                                                    
                                                    
                                                        Staff
                                                        Accrual Balances
                                                        Clock Times
                                                        Leave
                                                    
                                                
                                                
                                                Staff Locations
                                                Onboarding
                                            
                                        
                                        
                                        Departments
                                        
                                        Roles
                                        
                                        Qualifications
                                        
                                        Optional Allowances
                                        
                                        Colours
                                        
                                        Public Holidays
                                                 
                                        
                                            Audit
                                                
                                            
                                            
                                                Staff
                                                Roster
                                                Timesheet
                                            
                                        
                                           
                                        Security
                                        
                                        
                                            Demo
                                            
                                            
                                                Backup
                                                Restore
                                                Release IP Block
                                                Create Randomised Employees
                                            
                                                                                                         
                                    
                                
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        J
                                    
                                    
                                        
                                            
                                                J
                                            
                                            
                                                JerielQAjeriel_test@wageloch.com.au
                                            
                                            Manage your Wageloch User Account
                                            
                                        
                                        
                                            
                                                
                                                    Dark Mode
                                                    
                                                    
                                                
                                            
                                            
                                                
                                                    Colourblind Assist
                                                    
                                                    
                                                
                                            
                                        
                                        
                                            Sign Out
                                        
                                    
                                
                            
                        
                    
                
                
                    
                        
    Timesheet for the Week beginning February 9, 2026

                    
                    
                        
                        
	Central Payroll
	Central Site 1
	Central Site 2
	Central Site 3
	Central Site 4
	Stand Alone Site
	Wageloch Payroll


                    
                
                
                
                    
                    
                    
                    
                        
    
    


    .wl-grid-presubmit {
        height: calc(100vh - 280px - 45px);
    }

    .wl-rosterview-options input[type=&quot;radio&quot;]:checked + label {
        border-radius: 5px;
    }


	
    
		
        
            
            
			Barista
			Counter
			Kitchen
			Waitress
			Dishes
			Kitchenhand
			Chef
			Maintenance
			Marketing
			RSA
			RSP
			Manager
			Training
			Cook

		All selected (14)   Select all Barista Counter Kitchen Waitress Dishes Kitchenhand Chef Maintenance Marketing RSA RSP Manager Training Cook
        
    
	
    
        Pre-Submit
    
    
        
            
            
                
                
                    Save
                
                
                
                    Back
                
                
            
        
        
            
            
            
                Hide Empty Rows
            
            
                Hide Empty Columns
            
            
                
                    
                    
                        
                    
                
            
        
        
		
            
			
                            
                        
                            8.00
                        8.008.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        9.50
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        100.00
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        9.00
                            
                        
                            
                        
                            
                        
                            
                        8.00
                            
                        8.00
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        100.00
                            
                        100.00
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        8.00
                            
                        
                            
                        100.00
                            
                        80.00
                            
                        100.00
                            
                        
                            
                                                              
                            Banner, Bruce
                            
                            
                            
                        
                            Baratheon, Joffrey
                            
                            
                            
                        
                            Behm, Mirian
                            
                            
                            
                        
                            Billings, Heidi
                            
                            
                            
                        
                            Blanc, Koriks
                            
                            
                            
                        
                            Bouldin, Karren
                            
                            
                            
                        
                            Brogan, Neta
                            
                            
                            
                        
                            Cardamone, Tomas
                            
                            
                            
                        
                            Cashman, Ivonne
                            
                            
                            
                        
                            Chambermaid, King Prince
                            
                            
                            
                        
                            Cluggins, Fartrell
                            
                            
                            
                        
                            Corter, Elipses
                            
                            
                            
                        
                            Dookmarriot, L'Carpetron
                            
                            
                            
                        
                            Doughty, Luanne
                            
                            
                            
                        
                            Drake, Bobby
                            
                            
                            
                        
                            Drogo, Khal
                            
                            
                            
                        
                            Earhart, Edie
                            
                            
                            
                        
                            Employee333, Employee333
                            
                            
                            
                        
                            Employee500, Employee500
                            
                            
                            
                        
                            Frost, Emma
                            
                            
                            
                        
                            Funyuns, Bismo
                            
                            
                            
                        
                            Grey, Jean
                            
                            
                            
                        
                            Gulick, Cliff
                            
                            
                            
                        
                            Hemingway, Morris
                            
                            
                            
                        
                            Ikerd, Na
                            
                            
                            
                        
                            Ingles, Neta
                            
                            
                            
                        
                            Jilliumz, Leoz Maxwell
                            
                            
                            
                        
                            Jtc, Nob
                            
                            
                            
                        
                            Kent, Clark
                            
                            
                            
                        
                            Kopczynski, Carma
                            
                            
                            
                        
                            Lannister, Cersei
                            
                            
                            
                        
                            Lannister, Jamie
                            
                            
                            
                        
                            Lannister, Tyrion
                            
                            
                            
                        
                            LeBeau, Remy
                            
                            
                            
                        
                            Lewith, Torque
                            
                            
                            
                        
                            L'Goodling-Splatt, Swirvithan
                            
                            
                            
                        
                            Littler, Porsha
                            
                            
                            
                        
                            Looper, Elane
                            
                            
                            
                        
                            Machine, Age
                            
                            
                            
                        
                            Mangino, Glayds
                            
                            
                            
                        
                            Marcotte, Esperanza
                            
                            
                            
                        
                            Marte, Cliff
                            
                            
                            
                        
                            McCringleberry, Hingle
                            
                            
                            
                        
                            moz, zom
                            
                            
                            
                        
                            Munroe, Ororo
                            
                            
                            
                        
                            Oll, Red
                            
                            
                            
                        
                            Otte;:, Cathryn
                            
                            
                            
                        
                            Pineo, Aide
                            
                            
                            
                        
                            Pride, Kitty
                            
                            
                            
                        
                            Queen, Oliver
                            
                            
                            
                        
                            Rasputin, Piotr
                            
                            
                            
                        
                            Red, Oll
                            
                            
                            
                        
                            Roboclick, Stumptavian
                            
                            
                            
                        
                            Shower-Handel, Davoin
                            
                            
                            
                        
                            Snow, Jon
                            
                            
                            
                        
                            Soucie, Brian
                            
                            
                            
                        
                            Staffsn01, Stafffn01
                            
                            
                            
                        
                            staffsname, stafffname01
                            
                            
                            
                        
                            Staffsurname1, Staffname1
                            
                            
                            
                        
                            Stark, Arya
                            
                            
                            
                        
                            Stark, Sansa
                            
                            
                            
                        
                            Stock, Tomasa
                            
                            
                            
                        
                            Targaryen, Daenerys
                            
                            
                            
                        
                            Test123000, Test123000
                            
                            
                            
                        
                            test214214, test214214
                            
                            
                            
                        
                            Testemp00001, Testemp00001
                            
                            
                            
                        
                            Testemp00003, Testemp00003
                            
                            
                            
                        
                            Testemp00004, Testemp00004
                            
                            
                            
                        
                            Testemp00005, Testemp00005
                            
                            
                            
                        
                            Testemp00006, Testemp00006
                            
                            
                            
                        
                            Testemp00007, Testemp00007
                            
                            
                            
                        
                            Testemp00008, Testemp00008
                            
                            
                            
                        
                            Testemp00009, Testemp00009
                            
                            
                            
                        
                            Testemp00010, Testemp00010
                            
                            
                            
                        
                            Testemp00011, Testemp00011
                            
                            
                            
                        
                            Testemp00012, Testemp00012
                            
                            
                            
                        
                            Testemp00013, Testemp00013
                            
                            
                            
                        
                            Testemp00014, Testemp00014
                            
                            
                            
                        
                            Testemp00015, Testemp00015
                            
                            
                            
                        
                            Testemp00016, Testemp00016
                            
                            
                            
                        
                            Testemp00017, Testemp00017
                            
                            
                            
                        
                            Testemp00018, Testemp00018
                            
                            
                            
                        
                            Testemp00019, Testemp00019
                            
                            
                            
                        
                            Testemp00020, Testemp00020
                            
                            
                            
                        
                            Testemp00021, Testemp00021
                            
                            
                            
                        
                            Testemp00022, Testemp00022
                            
                            
                            
                        
                            Testemp00023, Testemp00023
                            
                            
                            
                        
                            Testemp00024, Testemp00024
                            
                            
                            
                        
                            Testemp00025, Testemp00025
                            
                            
                            
                        
                            Testemp00026, Testemp00026
                            
                            
                            
                        
                            Testemp00027, Testemp00027
                            
                            
                            
                        
                            Testemp00028, Testemp00028
                            
                            
                            
                        
                            Testemp00029, Testemp00029
                            
                            
                            
                        
                            Testemp00030, Testemp00030
                            
                            
                            
                        
                            Testemp00031, Testemp00031
                            
                            
                            
                        
                            Testemp00032, Testemp00032
                            
                            
                            
                        
                            Testemp00033, Testemp00033
                            
                            
                            
                        
                            Testemp00034, Testemp00034
                            
                            
                            
                        
                            Testemp00035, Testemp00035
                            
                            
                            
                        
                            Testemp00036, Testemp00036
                            
                            
                            
                        
                            Testemp00037, Testemp00037
                            
                            
                            
                        
                            Testemp00038, Testemp00038
                            
                            
                            
                        
                            Testemp00039, Testemp00039
                            
                            
                            
                        
                            Testemp00040, Testemp00040
                            
                            
                            
                        
                            Testemp00041, Testemp00041
                            
                            
                            
                        
                            Testemp00042, Testemp00042
                            
                            
                            
                        
                            Testemp00043, Testemp00043
                            
                            
                            
                        
                            Testemp00044, Testemp00044
                            
                            
                            
                        
                            Testemp00045, Testemp00045
                            
                            
                            
                        
                            Testemp00046, Testemp00046
                            
                            
                            
                        
                            Testemp00047, Testemp00047
                            
                            
                            
                        
                            Testemp00048, Testemp00048
                            
                            
                            
                        
                            Testemp00049, Testemp00049
                            
                            
                            
                        
                            Testemp00050, Testemp00050
                            
                            
                            
                        
                            TESTQA1084129745, TESTQA1084129745
                            
                            
                            
                        
                            TestQA108812742, TestQA108812742
                            
                            
                            
                        
                            TESTQA120983588, TESTQA120983588
                            
                            
                            
                        
                            Twitchell, Season
                            
                            
                            
                        
                            Vanderwal, Eula
                            
                            
                            
                        
                            Vanhoy, Mamie
                            
                            
                            
                        
                            Wagner, Kurt
                            
                            
                            
                        
                            Wayne, Bruce
                            
                            
                            
                        
                            Worthington III, Warren
                            
                            
                            
                        
                            Wrede, Heidi
                            
                            
                            
                        
                            Youssef, Rikki
                            
                            
                            
                         Total HoursTotal WorkedOrdM-FSatSunPub HolCas M-FCas SatCas SunCas L3 SunCas Pub HolCas Bef 7aCas Aft 10pBef 7aAft 10pO/T 1.5O/T 1.75O/T 2.0L2 M-FL2 SATL2 SUNL2 PHRESP M-FRESP SATRESP SUNRESP PHSUPERVISORMANAGERPHNoWorkANNUAL LVESICK LEAVEUNPAID LEAVETOIL PAIDWORKCOVERLSLTOIL ACCRUEDAlt HolidayStaff Name
		
        
	
        
    








    
    
    
    
    
    

                    
                    
                    
                    
                    
                
                
	
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                            
                        
                    
                

            
        
	
                
                    
                
            

        
	

    


//&lt;![CDATA[
webAppRosterStrips.myInit({parentID: 'MainContent_ucEditTimesheet_ctl01_ucRosterStrip_divStrips',rowHeight: '40',rosterByEmployee: 'False',createStaffBtn: 'False',btnStaffID: 'ctl00$MainContent$ucEditTimesheet$ctl01$ucRosterStrip$btnStaff',createStaffStrip: 'False',refreshAlert: 'False',isTimesheet: 'True',flipTimes: 'False',showRatio: 'False',minShiftMins: '30',labelTotalHrs: 'MainContent_ucEditTimesheet_ctl01_lblTotalHrs',labelTotalCost: 'MainContent_ucEditTimesheet_ctl01_lblTotalCost',isFromGrid: 'False',useDeptColors: 'False',useOptAllow: 'False',SnapPaid: 'True',OnlyShowWorkedHours: 'False',useBrk: 'False',brkMins: 0,insertBrkWhenLongerThan: 0,SplitBrkDifferential: 90,loadImg: true});webAppAPI.init('https://test.wageloch.com.au/WebAppApi.aspx/');initSportalApi();webAppUtils.myInit('aTimesheets');webAppSession.sessionExpireTimer(15, 'H0KhMU1aTk6QjM1HS1dnMvq4eiQp', false);webAppUtils.resetTimers(5);webAppMaster.registerBtnClocktimesEvent('btnClocktimes', '/clocktimelist');webAppMaster.registerHR('aHr', '/HRconfig');webAppMaster.registerLogoff('aLogout', 'hfChanged');webAppUtils.removeQueryString();webAppUtils.getGlobalAlertCntAPI('_tst_138051_5', 'bea3891d-d0dc-4382-bc11-f43b4b565b85');webAppRosterStrips.getRosterAlertCnt(true);webAppUtils.setupDaysScroll('MainContent_ucEditTimesheet_ctl01_rbDays','MainContent_ucEditTimesheet_ctl01_btnRbDaysLeft','MainContent_ucEditTimesheet_ctl01_btnRbDaysRight','MainContent_ucEditTimesheet_ctl01_hfScrollLeft');webAppRosterStrips.calcCosting();$.disposeCallbackDatas['MainContent_ucEditTimesheet_expEditTimesheet']='c1expander';$.disposeCallbackDatas['ctl00_ctl17']='c1gridview';
WebForm_InitCallback();Sys.Application.add_init(function() {
    jQuery(&quot;#MainContent_ucEditTimesheet_expEditTimesheet&quot;).c1expander(wijmoASPNetParseOptions({&quot;allowExpand&quot;: false, &quot;disabledState&quot;: false}));
});
Sys.Application.add_init(function() {
    $create(Sys.UI._UpdateProgress, {&quot;associatedUpdatePanelId&quot;:null,&quot;displayAfter&quot;:1,&quot;dynamicLayout&quot;:true}, null, null, $get(&quot;UpdatePp1&quot;));
});
Sys.Application.add_init(function() {
    jQuery(&quot;#ctl00_ctl17&quot;).c1gridview(wijmoASPNetParseOptions({&quot;culture&quot;: &quot;en-AU&quot;, &quot;pageIndex&quot;: 0, &quot;columns&quot;: [], &quot;pageCount&quot;: -1, &quot;showClientSelectionOnRender&quot;: true, &quot;innerState&quot;: {&quot;_pbRef&quot;: &quot;__doPostBack('ctl00$ctl17','{0}')&quot;, &quot;_cid&quot;: &quot;ctl00_ctl17&quot;, &quot;dbf&quot;: false, &quot;_uid&quot;: &quot;ctl00$ctl17&quot;, &quot;_bodyEditIndex&quot;: -1, &quot;scrollingState&quot;: {&quot;x&quot;:null,&quot;y&quot;:null,&quot;index&quot;:0}, &quot;_root&quot;: 1, &quot;_culture&quot;: &quot;en-AU&quot;, &quot;_bodySelectedIndex&quot;: -1, &quot;_dc&quot;: &quot;aspNetDisabled&quot;}, &quot;disabledState&quot;: false}));
});
//]]&gt;


    
        
            
                © 2024 - Wageloch
                
                
            
            
            
            
                window.dataLayer = window.dataLayer || [];
                function gtag() { dataLayer.push(arguments); }
                gtag('js', new Date());

                gtag('config', 'G-T6DX3BN0JJ');
            

            
                var x, i, j, selElmnt, a, b, c;
                /*look for any elements with the class &quot;custom-select&quot;:*/
                x = document.getElementsByClassName(&quot;custom-select&quot;);
                for (i = 0; i &lt; x.length; i++) {
                    selElmnt = x[i].getElementsByTagName(&quot;select&quot;)[0];
                    /*for each element, create a new DIV that will act as the selected item:*/
                    a = document.createElement(&quot;DIV&quot;);
                    a.setAttribute(&quot;class&quot;, &quot;select-selected&quot;);
                    a.innerHTML = selElmnt.options[selElmnt.selectedIndex].innerHTML;
                    x[i].appendChild(a);
                    /*for each element, create a new DIV that will contain the option list:*/
                    b = document.createElement(&quot;DIV&quot;);
                    b.setAttribute(&quot;class&quot;, &quot;select-items select-hide&quot;);
                    for (j = 1; j &lt; selElmnt.length; j++) {
                        /*for each option in the original select element,
                        create a new DIV that will act as an option item:*/
                        c = document.createElement(&quot;DIV&quot;);
                        c.innerHTML = selElmnt.options[j].innerHTML;
                        c.addEventListener(&quot;click&quot;, function (e) {
                            /*when an item is clicked, update the original select box,
                            and the selected item:*/
                            var y, i, k, s, h;
                            s = this.parentNode.parentNode.getElementsByTagName(&quot;select&quot;)[0];
                            h = this.parentNode.previousSibling;
                            for (i = 0; i &lt; s.length; i++) {
                                if (s.options[i].innerHTML == this.innerHTML) {
                                    s.selectedIndex = i;
                                    h.innerHTML = this.innerHTML;
                                    y = this.parentNode.getElementsByClassName(&quot;same-as-selected&quot;);
                                    for (k = 0; k &lt; y.length; k++) {
                                        y[k].removeAttribute(&quot;class&quot;);
                                    }
                                    this.setAttribute(&quot;class&quot;, &quot;same-as-selected&quot;);
                                    break;
                                }
                            }
                            h.click();
                        });
                        b.appendChild(c);
                    }
                    x[i].appendChild(b);
                    a.addEventListener(&quot;click&quot;, function (e) {
                        /*when the select box is clicked, close any other select boxes,
                        and open/close the current select box:*/
                        e.stopPropagation();
                        closeAllSelect(this);
                        this.nextSibling.classList.toggle(&quot;select-hide&quot;);
                        this.classList.toggle(&quot;select-arrow-active&quot;);
                    });
                }
                function closeAllSelect(elmnt) {
                    /*a function that will close all select boxes in the document,
                    except the current select box:*/
                    var x, y, i, arrNo = [];
                    x = document.getElementsByClassName(&quot;select-items&quot;);
                    y = document.getElementsByClassName(&quot;select-selected&quot;);
                    for (i = 0; i &lt; y.length; i++) {
                        if (elmnt == y[i]) {
                            arrNo.push(i)
                        } else {
                            y[i].classList.remove(&quot;select-arrow-active&quot;);
                        }
                    }
                    for (i = 0; i &lt; x.length; i++) {
                        if (arrNo.indexOf(i)) {
                            x[i].classList.add(&quot;select-hide&quot;);
                        }
                    }
                }
                /*if the user clicks anywhere outside the select box,
                then close all select boxes:*/
                document.addEventListener(&quot;click&quot;, closeAllSelect);
            
        
    
    
        
            
                Cookies Required
                Cookies are not enabled on your browser. Please enable cookies in your browser settings to continue.
            
        
    



#chatbase-bubble-button,
#chatbase-bubble-button * {
 margin: 0;
 padding: 0;
 box-sizing: border-box;
}
	✕Hi, I'm Hugo. How can I help you today?/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]</value>
      <webElementGuid>df36fa73-cb09-4549-945e-2399a540fcf9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]</value>
      <webElementGuid>81a8d320-438d-41ad-9603-3279758b7af7</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>6593c82f-a9ac-4c32-b38a-b4127675dff0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>b26cd326-4497-40b0-b925-cd1f8d3c948b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;Wageloch - Edit Timesheet






        
        























    
    
    
        let webAppPreSubmit = (function () {
            let scrollPos = 0;
            function init() {
                setupPayItemsToggle();
                const btnSearch = document.getElementById(&quot; , &quot;'&quot; , &quot;btnSearch&quot; , &quot;'&quot; , &quot;);
                if (btnSearch != null) btnSearch.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, staffRecordScrollIntoView, true);
                const preSubmitList = document.getElementById(&quot; , &quot;'&quot; , &quot;preSubmitList&quot; , &quot;'&quot; , &quot;);
                if (preSubmitList != null) preSubmitList.addEventListener(&quot; , &quot;'&quot; , &quot;scroll&quot; , &quot;'&quot; , &quot;, getScrollPos, true);
                $(&quot; , &quot;'&quot; , &quot;[data-wl-payitemhrs]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function (e) { validateHrs(e); });
                setScrollPos();
            }
            function setupPayItemsToggle() {
                const d = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-wl-payitemheader]&quot; , &quot;'&quot; , &quot;);
                for (let i = 0, l = d.length; i &lt; l; i++) {
                    const ele = d[i];
                    const attr = ele.getAttribute(&quot; , &quot;'&quot; , &quot;data-wl-payitemheader&quot; , &quot;'&quot; , &quot;);
                    ele.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () { togglePayItem(this, attr); }, false);
                }
            }
            function togglePayItem(ele, attr) {
                const t = ele.getAttribute(&quot; , &quot;'&quot; , &quot;data-wl-toggle&quot; , &quot;'&quot; , &quot;);
                const header = attr == &quot; , &quot;'&quot; , &quot;allpayitems&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;[data-wl-payitemheader]&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;[data-wl-payitemheader=&quot;&quot; , &quot;'&quot; , &quot; + attr + &quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;;
                const selector = attr == &quot; , &quot;'&quot; , &quot;allpayitems&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;[data-wl-payitems]&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;[data-wl-payitems=&quot;&quot; , &quot;'&quot; , &quot; + attr + &quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;;
                const i = attr == &quot; , &quot;'&quot; , &quot;allpayitems&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;[data-wl-togglei]&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;[data-wl-togglei=&quot;&quot; , &quot;'&quot; , &quot; + attr + &quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;;
                if (t == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
                    $(header).attr(&quot; , &quot;'&quot; , &quot;data-wl-toggle&quot; , &quot;'&quot; , &quot;, 0);
                    $(i).removeClass(&quot; , &quot;'&quot; , &quot;fa-caret-down&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;fa-caret-up&quot; , &quot;'&quot; , &quot;);
                    $(selector).slideDown();
                } else {
                    $(header).attr(&quot; , &quot;'&quot; , &quot;data-wl-toggle&quot; , &quot;'&quot; , &quot;, 1);
                    $(i).addClass(&quot; , &quot;'&quot; , &quot;fa-caret-down&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;fa-caret-up&quot; , &quot;'&quot; , &quot;);
                    $(selector).slideUp();
                }
            }
            function staffRecordScrollIntoView() {
                let lowerCaseValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                const t = document.querySelector(&quot; , &quot;'&quot; , &quot;[data-wl-searchinput=&quot;staff&quot;]&quot; , &quot;'&quot; , &quot;);
                if (t != null) lowerCaseValue = t.value.toLowerCase();
                const emps = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-wl-empname]&quot; , &quot;'&quot; , &quot;);
                for (let i = 0, l = emps.length; i &lt; l; i++) {
                    const emp = emps[i];
                    const name = emp.getAttribute(&quot; , &quot;'&quot; , &quot;data-wl-empname&quot; , &quot;'&quot; , &quot;);
                    if (name.toLowerCase().includes(lowerCaseValue)) {
                        emp.scrollIntoView();
                        break;
                    }
                }
            }
            function getScrollPos() {
                const preSubmitList = document.getElementById(&quot; , &quot;'&quot; , &quot;preSubmitList&quot; , &quot;'&quot; , &quot;);
                if (preSubmitList != null) scrollPos = preSubmitList.scrollTop;
            }
            function setScrollPos() {
                const preSubmitList = document.getElementById(&quot; , &quot;'&quot; , &quot;preSubmitList&quot; , &quot;'&quot; , &quot;);
                if (preSubmitList != null) preSubmitList.scrollTop = scrollPos;
            }
            function validateHrs(e) {
                const t = e.target;
                const v = t.value;
                if (v &lt; 0) t.value = 0;
            }
            return {
                init: init
            }
        }());
    



    
        /*.badge-notify {
            background: red;
            position: relative !important;
            top: -12px;
            left: -15px;
            font-size-adjust: 0.4;
        }*/

        #xCentralisedSiteSelection {
            max-width: 250px;
            float: right;
            margin-right: 20px;
            z-index: 9999;
            position: absolute;
            top: 100px;
            right: 0;
        }
        /*the container must be positioned relative:*/
        .custom-select {
            position: relative;
            font-family: Arial;
        }

            .custom-select select {
                display: none; /*hide original SELECT element:*/
            }

        .select-selected {
            background-color: DodgerBlue;
        }
            /*style the arrow inside the select element:*/
            .select-selected:after {
                position: absolute;
                content: &quot;&quot;;
                top: 14px;
                right: 10px;
                width: 0;
                height: 0;
                border: 6px solid transparent;
                border-color: #fff transparent transparent transparent;
            }
            /*point the arrow upwards when the select box is open (active):*/
            .select-selected.select-arrow-active:after {
                border-color: transparent transparent #fff transparent;
                top: 7px;
            }
        /*style the items (options), including the selected item:*/
        .select-items div, .select-selected {
            color: #ffffff;
            padding: 8px 16px;
            border: 1px solid transparent;
            border-color: transparent transparent rgba(0, 0, 0, 0.1) transparent;
            cursor: pointer;
        }
        /*style items (options):*/
        .select-items {
            position: absolute;
            background-color: DodgerBlue;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 99;
        }
        /*hide the items when the select box is closed:*/
        .select-hide {
            display: none;
        }

        .select-items div:hover, .same-as-selected {
            background-color: rgba(0, 0, 0, 0.1);
        }
    
    
        function showhelp() {
            myWindow = window.open(&quot;/ShowHelp&quot;, &quot;_help&quot;, &quot;width=800, height=600&quot;);
        }
        let webAppMaster = function () {
            function registerBtnClocktimesEvent(btnID, sLink) {
                $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + btnID).click(function (e) {
                    window.open(sLink, &quot;ClockTimes&quot;);
                    this.blur();
                    return false;
                });
            }
            function registerHR(btnID, sLink) {
                $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + btnID).click(function (e) {
                    window.open(sLink, &quot;HR&quot;);
                    this.blur();
                    return false;
                });
            }
            function registerLogoff(btnID, changeID) {
                $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + btnID).click(function (e) {
                    const c = document.getElementById(changeID);
                    if (c != null) {
                        const changed = webAppUtils.parseBool(c.value);
                        if (changed) {
                            webAppUtils.msgShow(&quot; , &quot;'&quot; , &quot;Sign Out&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Changes that you made will not be saved.&lt;br>&lt;br>Do you still want to continue?&quot; , &quot;'&quot; , &quot;, webAppConfig.MsgBoxStyle.YesNo, webAppSession.logoutAllPages);
                        } else
                            webAppSession.logoutAllPages();
                    }
                    return false;
                });
                const a = document.getElementById(&quot; , &quot;'&quot; , &quot;chkAAAMode&quot; , &quot;'&quot; , &quot;);
                if (a != null) a.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () { webAppUtils.aaaModeAPI(this.checked); }, false);
                const d = document.getElementById(&quot; , &quot;'&quot; , &quot;chkDarkMode&quot; , &quot;'&quot; , &quot;);
                if (d != null) d.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () { webAppUtils.darkModeAPI(this.checked); }, false);
            }
            return {
                registerBtnClocktimesEvent: registerBtnClocktimesEvent,
                registerHR: registerHR,
                registerLogoff: registerLogoff
            }
        }();
    
    
        window.embeddedChatbotConfig = {
            chatbotId: &quot;DK_sXVyG_1_XSidG5zaMQ&quot;,
            domain: &quot;www.chatbase.co&quot;
        }
    
    
    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

    
    









//&lt;![CDATA[
var theForm = document.forms[&quot; , &quot;'&quot; , &quot;frmWebApp&quot; , &quot;'&quot; , &quot;];
if (!theForm) {
    theForm = document.frmWebApp;
}
function __doPostBack(eventTarget, eventArgument) {
    if (!theForm.onsubmit || (theForm.onsubmit() != false)) {
        theForm.__EVENTTARGET.value = eventTarget;
        theForm.__EVENTARGUMENT.value = eventArgument;
        theForm.submit();
    }
}
//]]&gt;







//&lt;![CDATA[
if (typeof(Sys) === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) throw new Error(&quot; , &quot;'&quot; , &quot;ASP.NET Ajax client-side framework failed to load.&quot; , &quot;'&quot; , &quot;);
//]]&gt;






//&lt;![CDATA[
function WebForm_OnSubmit() {
if (webAppSession.sessionExpired()) return false;webAppUtils.removeElement(webAppConfig.ID.ROSTERSTRIP_TABLE_BODY);$(&quot; , &quot;'&quot; , &quot;#MainContent_ucEditTimesheet_expEditTimesheet&quot; , &quot;'&quot; , &quot;).wijSaveState(&quot; , &quot;'&quot; , &quot;c1expander&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#ctl00_ctl17&quot; , &quot;'&quot; , &quot;).wijSaveState(&quot; , &quot;'&quot; , &quot;c1gridview&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onwijstatesaving&quot; , &quot;'&quot; , &quot;);
return true;
}
//]]&gt;




	
	

        
        
        
        
        
//&lt;![CDATA[
Sys.WebForms.PageRequestManager._initialize(&quot; , &quot;'&quot; , &quot;ctl00$ScriptManager1&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;frmWebApp&quot; , &quot;'&quot; , &quot;, [&quot; , &quot;'&quot; , &quot;tctl00$UpdatePanelMain&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;UpdatePanelMain&quot; , &quot;'&quot; , &quot;], [], [], 600, &quot; , &quot;'&quot; , &quot;ctl00&quot; , &quot;'&quot; , &quot;);
//]]&gt;


        
                
                
                
                
                    
                        
                            
                                
                                
                                
                            
                            
                        
                        
                            
                                
                                    Dashboard
                                
                                
                                    
                                
                                
                                    Rosters
                                
                                
                                    Timesheets
                                
                                
                                    Staff
                                
                                
                                    Clock Times
                                
                                
                                    Reports
                                
                                
                                    HR
                                
                            
                            
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                            
                                        
                                    
                                
                                
                                    
                                        
                                        
                                    
                                    
                                        
                                        General
                                            
                                        
                                        Roster
                                        
                                        Timesheet
                                        
                                        
                                        
                                        
                                            Portal/Mobile App
                                           
                                            
                                            
                                                Leave
                                                Bulk Mobile Staff Clock
                                                Mobile Proximity Settings
                                                Portal Setup
                                            
                                        
                                        
                                        
                                            Payroll
                                        
                                            
                                            
                                                Pay Level
                                                Payroll Connection
                                                
                                                Payroll Mappings
                                                    
                                                
                                                
                                            
                                        
                                        
                                        
                                            Staff
                                                
                                            
                                            
                                                
                                                    Import
                                                        
                                                    
                                                    
                                                        Staff
                                                        Accrual Balances
                                                        Clock Times
                                                        Leave
                                                    
                                                
                                                
                                                Staff Locations
                                                Onboarding
                                            
                                        
                                        
                                        Departments
                                        
                                        Roles
                                        
                                        Qualifications
                                        
                                        Optional Allowances
                                        
                                        Colours
                                        
                                        Public Holidays
                                                 
                                        
                                            Audit
                                                
                                            
                                            
                                                Staff
                                                Roster
                                                Timesheet
                                            
                                        
                                           
                                        Security
                                        
                                        
                                            Demo
                                            
                                            
                                                Backup
                                                Restore
                                                Release IP Block
                                                Create Randomised Employees
                                            
                                                                                                         
                                    
                                
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        J
                                    
                                    
                                        
                                            
                                                J
                                            
                                            
                                                JerielQAjeriel_test@wageloch.com.au
                                            
                                            Manage your Wageloch User Account
                                            
                                        
                                        
                                            
                                                
                                                    Dark Mode
                                                    
                                                    
                                                
                                            
                                            
                                                
                                                    Colourblind Assist
                                                    
                                                    
                                                
                                            
                                        
                                        
                                            Sign Out
                                        
                                    
                                
                            
                        
                    
                
                
                    
                        
    Timesheet for the Week beginning February 9, 2026

                    
                    
                        
                        
	Central Payroll
	Central Site 1
	Central Site 2
	Central Site 3
	Central Site 4
	Stand Alone Site
	Wageloch Payroll


                    
                
                
                
                    
                    
                    
                    
                        
    
    


    .wl-grid-presubmit {
        height: calc(100vh - 280px - 45px);
    }

    .wl-rosterview-options input[type=&quot;radio&quot;]:checked + label {
        border-radius: 5px;
    }


	
    
		
        
            
            
			Barista
			Counter
			Kitchen
			Waitress
			Dishes
			Kitchenhand
			Chef
			Maintenance
			Marketing
			RSA
			RSP
			Manager
			Training
			Cook

		All selected (14)   Select all Barista Counter Kitchen Waitress Dishes Kitchenhand Chef Maintenance Marketing RSA RSP Manager Training Cook
        
    
	
    
        Pre-Submit
    
    
        
            
            
                
                
                    Save
                
                
                
                    Back
                
                
            
        
        
            
            
            
                Hide Empty Rows
            
            
                Hide Empty Columns
            
            
                
                    
                    
                        
                    
                
            
        
        
		
            
			
                            
                        
                            8.00
                        8.008.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        9.50
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        100.00
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        9.00
                            
                        
                            
                        
                            
                        
                            
                        8.00
                            
                        8.00
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        100.00
                            
                        100.00
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        8.00
                            
                        
                            
                        100.00
                            
                        80.00
                            
                        100.00
                            
                        
                            
                                                              
                            Banner, Bruce
                            
                            
                            
                        
                            Baratheon, Joffrey
                            
                            
                            
                        
                            Behm, Mirian
                            
                            
                            
                        
                            Billings, Heidi
                            
                            
                            
                        
                            Blanc, Koriks
                            
                            
                            
                        
                            Bouldin, Karren
                            
                            
                            
                        
                            Brogan, Neta
                            
                            
                            
                        
                            Cardamone, Tomas
                            
                            
                            
                        
                            Cashman, Ivonne
                            
                            
                            
                        
                            Chambermaid, King Prince
                            
                            
                            
                        
                            Cluggins, Fartrell
                            
                            
                            
                        
                            Corter, Elipses
                            
                            
                            
                        
                            Dookmarriot, L&quot; , &quot;'&quot; , &quot;Carpetron
                            
                            
                            
                        
                            Doughty, Luanne
                            
                            
                            
                        
                            Drake, Bobby
                            
                            
                            
                        
                            Drogo, Khal
                            
                            
                            
                        
                            Earhart, Edie
                            
                            
                            
                        
                            Employee333, Employee333
                            
                            
                            
                        
                            Employee500, Employee500
                            
                            
                            
                        
                            Frost, Emma
                            
                            
                            
                        
                            Funyuns, Bismo
                            
                            
                            
                        
                            Grey, Jean
                            
                            
                            
                        
                            Gulick, Cliff
                            
                            
                            
                        
                            Hemingway, Morris
                            
                            
                            
                        
                            Ikerd, Na
                            
                            
                            
                        
                            Ingles, Neta
                            
                            
                            
                        
                            Jilliumz, Leoz Maxwell
                            
                            
                            
                        
                            Jtc, Nob
                            
                            
                            
                        
                            Kent, Clark
                            
                            
                            
                        
                            Kopczynski, Carma
                            
                            
                            
                        
                            Lannister, Cersei
                            
                            
                            
                        
                            Lannister, Jamie
                            
                            
                            
                        
                            Lannister, Tyrion
                            
                            
                            
                        
                            LeBeau, Remy
                            
                            
                            
                        
                            Lewith, Torque
                            
                            
                            
                        
                            L&quot; , &quot;'&quot; , &quot;Goodling-Splatt, Swirvithan
                            
                            
                            
                        
                            Littler, Porsha
                            
                            
                            
                        
                            Looper, Elane
                            
                            
                            
                        
                            Machine, Age
                            
                            
                            
                        
                            Mangino, Glayds
                            
                            
                            
                        
                            Marcotte, Esperanza
                            
                            
                            
                        
                            Marte, Cliff
                            
                            
                            
                        
                            McCringleberry, Hingle
                            
                            
                            
                        
                            moz, zom
                            
                            
                            
                        
                            Munroe, Ororo
                            
                            
                            
                        
                            Oll, Red
                            
                            
                            
                        
                            Otte;:, Cathryn
                            
                            
                            
                        
                            Pineo, Aide
                            
                            
                            
                        
                            Pride, Kitty
                            
                            
                            
                        
                            Queen, Oliver
                            
                            
                            
                        
                            Rasputin, Piotr
                            
                            
                            
                        
                            Red, Oll
                            
                            
                            
                        
                            Roboclick, Stumptavian
                            
                            
                            
                        
                            Shower-Handel, Davoin
                            
                            
                            
                        
                            Snow, Jon
                            
                            
                            
                        
                            Soucie, Brian
                            
                            
                            
                        
                            Staffsn01, Stafffn01
                            
                            
                            
                        
                            staffsname, stafffname01
                            
                            
                            
                        
                            Staffsurname1, Staffname1
                            
                            
                            
                        
                            Stark, Arya
                            
                            
                            
                        
                            Stark, Sansa
                            
                            
                            
                        
                            Stock, Tomasa
                            
                            
                            
                        
                            Targaryen, Daenerys
                            
                            
                            
                        
                            Test123000, Test123000
                            
                            
                            
                        
                            test214214, test214214
                            
                            
                            
                        
                            Testemp00001, Testemp00001
                            
                            
                            
                        
                            Testemp00003, Testemp00003
                            
                            
                            
                        
                            Testemp00004, Testemp00004
                            
                            
                            
                        
                            Testemp00005, Testemp00005
                            
                            
                            
                        
                            Testemp00006, Testemp00006
                            
                            
                            
                        
                            Testemp00007, Testemp00007
                            
                            
                            
                        
                            Testemp00008, Testemp00008
                            
                            
                            
                        
                            Testemp00009, Testemp00009
                            
                            
                            
                        
                            Testemp00010, Testemp00010
                            
                            
                            
                        
                            Testemp00011, Testemp00011
                            
                            
                            
                        
                            Testemp00012, Testemp00012
                            
                            
                            
                        
                            Testemp00013, Testemp00013
                            
                            
                            
                        
                            Testemp00014, Testemp00014
                            
                            
                            
                        
                            Testemp00015, Testemp00015
                            
                            
                            
                        
                            Testemp00016, Testemp00016
                            
                            
                            
                        
                            Testemp00017, Testemp00017
                            
                            
                            
                        
                            Testemp00018, Testemp00018
                            
                            
                            
                        
                            Testemp00019, Testemp00019
                            
                            
                            
                        
                            Testemp00020, Testemp00020
                            
                            
                            
                        
                            Testemp00021, Testemp00021
                            
                            
                            
                        
                            Testemp00022, Testemp00022
                            
                            
                            
                        
                            Testemp00023, Testemp00023
                            
                            
                            
                        
                            Testemp00024, Testemp00024
                            
                            
                            
                        
                            Testemp00025, Testemp00025
                            
                            
                            
                        
                            Testemp00026, Testemp00026
                            
                            
                            
                        
                            Testemp00027, Testemp00027
                            
                            
                            
                        
                            Testemp00028, Testemp00028
                            
                            
                            
                        
                            Testemp00029, Testemp00029
                            
                            
                            
                        
                            Testemp00030, Testemp00030
                            
                            
                            
                        
                            Testemp00031, Testemp00031
                            
                            
                            
                        
                            Testemp00032, Testemp00032
                            
                            
                            
                        
                            Testemp00033, Testemp00033
                            
                            
                            
                        
                            Testemp00034, Testemp00034
                            
                            
                            
                        
                            Testemp00035, Testemp00035
                            
                            
                            
                        
                            Testemp00036, Testemp00036
                            
                            
                            
                        
                            Testemp00037, Testemp00037
                            
                            
                            
                        
                            Testemp00038, Testemp00038
                            
                            
                            
                        
                            Testemp00039, Testemp00039
                            
                            
                            
                        
                            Testemp00040, Testemp00040
                            
                            
                            
                        
                            Testemp00041, Testemp00041
                            
                            
                            
                        
                            Testemp00042, Testemp00042
                            
                            
                            
                        
                            Testemp00043, Testemp00043
                            
                            
                            
                        
                            Testemp00044, Testemp00044
                            
                            
                            
                        
                            Testemp00045, Testemp00045
                            
                            
                            
                        
                            Testemp00046, Testemp00046
                            
                            
                            
                        
                            Testemp00047, Testemp00047
                            
                            
                            
                        
                            Testemp00048, Testemp00048
                            
                            
                            
                        
                            Testemp00049, Testemp00049
                            
                            
                            
                        
                            Testemp00050, Testemp00050
                            
                            
                            
                        
                            TESTQA1084129745, TESTQA1084129745
                            
                            
                            
                        
                            TestQA108812742, TestQA108812742
                            
                            
                            
                        
                            TESTQA120983588, TESTQA120983588
                            
                            
                            
                        
                            Twitchell, Season
                            
                            
                            
                        
                            Vanderwal, Eula
                            
                            
                            
                        
                            Vanhoy, Mamie
                            
                            
                            
                        
                            Wagner, Kurt
                            
                            
                            
                        
                            Wayne, Bruce
                            
                            
                            
                        
                            Worthington III, Warren
                            
                            
                            
                        
                            Wrede, Heidi
                            
                            
                            
                        
                            Youssef, Rikki
                            
                            
                            
                         Total HoursTotal WorkedOrdM-FSatSunPub HolCas M-FCas SatCas SunCas L3 SunCas Pub HolCas Bef 7aCas Aft 10pBef 7aAft 10pO/T 1.5O/T 1.75O/T 2.0L2 M-FL2 SATL2 SUNL2 PHRESP M-FRESP SATRESP SUNRESP PHSUPERVISORMANAGERPHNoWorkANNUAL LVESICK LEAVEUNPAID LEAVETOIL PAIDWORKCOVERLSLTOIL ACCRUEDAlt HolidayStaff Name
		
        
	
        
    








    
    
    
    
    
    

                    
                    
                    
                    
                    
                
                
	
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                            
                        
                    
                

            
        
	
                
                    
                
            

        
	

    


//&lt;![CDATA[
webAppRosterStrips.myInit({parentID: &quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_ucRosterStrip_divStrips&quot; , &quot;'&quot; , &quot;,rowHeight: &quot; , &quot;'&quot; , &quot;40&quot; , &quot;'&quot; , &quot;,rosterByEmployee: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,createStaffBtn: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,btnStaffID: &quot; , &quot;'&quot; , &quot;ctl00$MainContent$ucEditTimesheet$ctl01$ucRosterStrip$btnStaff&quot; , &quot;'&quot; , &quot;,createStaffStrip: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,refreshAlert: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,isTimesheet: &quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;,flipTimes: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,showRatio: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,minShiftMins: &quot; , &quot;'&quot; , &quot;30&quot; , &quot;'&quot; , &quot;,labelTotalHrs: &quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_lblTotalHrs&quot; , &quot;'&quot; , &quot;,labelTotalCost: &quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_lblTotalCost&quot; , &quot;'&quot; , &quot;,isFromGrid: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,useDeptColors: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,useOptAllow: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,SnapPaid: &quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;,OnlyShowWorkedHours: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,useBrk: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,brkMins: 0,insertBrkWhenLongerThan: 0,SplitBrkDifferential: 90,loadImg: true});webAppAPI.init(&quot; , &quot;'&quot; , &quot;https://test.wageloch.com.au/WebAppApi.aspx/&quot; , &quot;'&quot; , &quot;);initSportalApi();webAppUtils.myInit(&quot; , &quot;'&quot; , &quot;aTimesheets&quot; , &quot;'&quot; , &quot;);webAppSession.sessionExpireTimer(15, &quot; , &quot;'&quot; , &quot;H0KhMU1aTk6QjM1HS1dnMvq4eiQp&quot; , &quot;'&quot; , &quot;, false);webAppUtils.resetTimers(5);webAppMaster.registerBtnClocktimesEvent(&quot; , &quot;'&quot; , &quot;btnClocktimes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/clocktimelist&quot; , &quot;'&quot; , &quot;);webAppMaster.registerHR(&quot; , &quot;'&quot; , &quot;aHr&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/HRconfig&quot; , &quot;'&quot; , &quot;);webAppMaster.registerLogoff(&quot; , &quot;'&quot; , &quot;aLogout&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;hfChanged&quot; , &quot;'&quot; , &quot;);webAppUtils.removeQueryString();webAppUtils.getGlobalAlertCntAPI(&quot; , &quot;'&quot; , &quot;_tst_138051_5&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;bea3891d-d0dc-4382-bc11-f43b4b565b85&quot; , &quot;'&quot; , &quot;);webAppRosterStrips.getRosterAlertCnt(true);webAppUtils.setupDaysScroll(&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_rbDays&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_btnRbDaysLeft&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_btnRbDaysRight&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_hfScrollLeft&quot; , &quot;'&quot; , &quot;);webAppRosterStrips.calcCosting();$.disposeCallbackDatas[&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_expEditTimesheet&quot; , &quot;'&quot; , &quot;]=&quot; , &quot;'&quot; , &quot;c1expander&quot; , &quot;'&quot; , &quot;;$.disposeCallbackDatas[&quot; , &quot;'&quot; , &quot;ctl00_ctl17&quot; , &quot;'&quot; , &quot;]=&quot; , &quot;'&quot; , &quot;c1gridview&quot; , &quot;'&quot; , &quot;;
WebForm_InitCallback();Sys.Application.add_init(function() {
    jQuery(&quot;#MainContent_ucEditTimesheet_expEditTimesheet&quot;).c1expander(wijmoASPNetParseOptions({&quot;allowExpand&quot;: false, &quot;disabledState&quot;: false}));
});
Sys.Application.add_init(function() {
    $create(Sys.UI._UpdateProgress, {&quot;associatedUpdatePanelId&quot;:null,&quot;displayAfter&quot;:1,&quot;dynamicLayout&quot;:true}, null, null, $get(&quot;UpdatePp1&quot;));
});
Sys.Application.add_init(function() {
    jQuery(&quot;#ctl00_ctl17&quot;).c1gridview(wijmoASPNetParseOptions({&quot;culture&quot;: &quot;en-AU&quot;, &quot;pageIndex&quot;: 0, &quot;columns&quot;: [], &quot;pageCount&quot;: -1, &quot;showClientSelectionOnRender&quot;: true, &quot;innerState&quot;: {&quot;_pbRef&quot;: &quot;__doPostBack(&quot; , &quot;'&quot; , &quot;ctl00$ctl17&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;{0}&quot; , &quot;'&quot; , &quot;)&quot;, &quot;_cid&quot;: &quot;ctl00_ctl17&quot;, &quot;dbf&quot;: false, &quot;_uid&quot;: &quot;ctl00$ctl17&quot;, &quot;_bodyEditIndex&quot;: -1, &quot;scrollingState&quot;: {&quot;x&quot;:null,&quot;y&quot;:null,&quot;index&quot;:0}, &quot;_root&quot;: 1, &quot;_culture&quot;: &quot;en-AU&quot;, &quot;_bodySelectedIndex&quot;: -1, &quot;_dc&quot;: &quot;aspNetDisabled&quot;}, &quot;disabledState&quot;: false}));
});
//]]&gt;


    
        
            
                © 2024 - Wageloch
                
                
            
            
            
            
                window.dataLayer = window.dataLayer || [];
                function gtag() { dataLayer.push(arguments); }
                gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());

                gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;G-T6DX3BN0JJ&quot; , &quot;'&quot; , &quot;);
            

            
                var x, i, j, selElmnt, a, b, c;
                /*look for any elements with the class &quot;custom-select&quot;:*/
                x = document.getElementsByClassName(&quot;custom-select&quot;);
                for (i = 0; i &lt; x.length; i++) {
                    selElmnt = x[i].getElementsByTagName(&quot;select&quot;)[0];
                    /*for each element, create a new DIV that will act as the selected item:*/
                    a = document.createElement(&quot;DIV&quot;);
                    a.setAttribute(&quot;class&quot;, &quot;select-selected&quot;);
                    a.innerHTML = selElmnt.options[selElmnt.selectedIndex].innerHTML;
                    x[i].appendChild(a);
                    /*for each element, create a new DIV that will contain the option list:*/
                    b = document.createElement(&quot;DIV&quot;);
                    b.setAttribute(&quot;class&quot;, &quot;select-items select-hide&quot;);
                    for (j = 1; j &lt; selElmnt.length; j++) {
                        /*for each option in the original select element,
                        create a new DIV that will act as an option item:*/
                        c = document.createElement(&quot;DIV&quot;);
                        c.innerHTML = selElmnt.options[j].innerHTML;
                        c.addEventListener(&quot;click&quot;, function (e) {
                            /*when an item is clicked, update the original select box,
                            and the selected item:*/
                            var y, i, k, s, h;
                            s = this.parentNode.parentNode.getElementsByTagName(&quot;select&quot;)[0];
                            h = this.parentNode.previousSibling;
                            for (i = 0; i &lt; s.length; i++) {
                                if (s.options[i].innerHTML == this.innerHTML) {
                                    s.selectedIndex = i;
                                    h.innerHTML = this.innerHTML;
                                    y = this.parentNode.getElementsByClassName(&quot;same-as-selected&quot;);
                                    for (k = 0; k &lt; y.length; k++) {
                                        y[k].removeAttribute(&quot;class&quot;);
                                    }
                                    this.setAttribute(&quot;class&quot;, &quot;same-as-selected&quot;);
                                    break;
                                }
                            }
                            h.click();
                        });
                        b.appendChild(c);
                    }
                    x[i].appendChild(b);
                    a.addEventListener(&quot;click&quot;, function (e) {
                        /*when the select box is clicked, close any other select boxes,
                        and open/close the current select box:*/
                        e.stopPropagation();
                        closeAllSelect(this);
                        this.nextSibling.classList.toggle(&quot;select-hide&quot;);
                        this.classList.toggle(&quot;select-arrow-active&quot;);
                    });
                }
                function closeAllSelect(elmnt) {
                    /*a function that will close all select boxes in the document,
                    except the current select box:*/
                    var x, y, i, arrNo = [];
                    x = document.getElementsByClassName(&quot;select-items&quot;);
                    y = document.getElementsByClassName(&quot;select-selected&quot;);
                    for (i = 0; i &lt; y.length; i++) {
                        if (elmnt == y[i]) {
                            arrNo.push(i)
                        } else {
                            y[i].classList.remove(&quot;select-arrow-active&quot;);
                        }
                    }
                    for (i = 0; i &lt; x.length; i++) {
                        if (arrNo.indexOf(i)) {
                            x[i].classList.add(&quot;select-hide&quot;);
                        }
                    }
                }
                /*if the user clicks anywhere outside the select box,
                then close all select boxes:*/
                document.addEventListener(&quot;click&quot;, closeAllSelect);
            
        
    
    
        
            
                Cookies Required
                Cookies are not enabled on your browser. Please enable cookies in your browser settings to continue.
            
        
    



#chatbase-bubble-button,
#chatbase-bubble-button * {
 margin: 0;
 padding: 0;
 box-sizing: border-box;
}
	✕Hi, I&quot; , &quot;'&quot; , &quot;m Hugo. How can I help you today?/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]&quot;) or . = concat(&quot;Wageloch - Edit Timesheet






        
        























    
    
    
        let webAppPreSubmit = (function () {
            let scrollPos = 0;
            function init() {
                setupPayItemsToggle();
                const btnSearch = document.getElementById(&quot; , &quot;'&quot; , &quot;btnSearch&quot; , &quot;'&quot; , &quot;);
                if (btnSearch != null) btnSearch.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, staffRecordScrollIntoView, true);
                const preSubmitList = document.getElementById(&quot; , &quot;'&quot; , &quot;preSubmitList&quot; , &quot;'&quot; , &quot;);
                if (preSubmitList != null) preSubmitList.addEventListener(&quot; , &quot;'&quot; , &quot;scroll&quot; , &quot;'&quot; , &quot;, getScrollPos, true);
                $(&quot; , &quot;'&quot; , &quot;[data-wl-payitemhrs]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function (e) { validateHrs(e); });
                setScrollPos();
            }
            function setupPayItemsToggle() {
                const d = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-wl-payitemheader]&quot; , &quot;'&quot; , &quot;);
                for (let i = 0, l = d.length; i &lt; l; i++) {
                    const ele = d[i];
                    const attr = ele.getAttribute(&quot; , &quot;'&quot; , &quot;data-wl-payitemheader&quot; , &quot;'&quot; , &quot;);
                    ele.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () { togglePayItem(this, attr); }, false);
                }
            }
            function togglePayItem(ele, attr) {
                const t = ele.getAttribute(&quot; , &quot;'&quot; , &quot;data-wl-toggle&quot; , &quot;'&quot; , &quot;);
                const header = attr == &quot; , &quot;'&quot; , &quot;allpayitems&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;[data-wl-payitemheader]&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;[data-wl-payitemheader=&quot;&quot; , &quot;'&quot; , &quot; + attr + &quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;;
                const selector = attr == &quot; , &quot;'&quot; , &quot;allpayitems&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;[data-wl-payitems]&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;[data-wl-payitems=&quot;&quot; , &quot;'&quot; , &quot; + attr + &quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;;
                const i = attr == &quot; , &quot;'&quot; , &quot;allpayitems&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;[data-wl-togglei]&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;[data-wl-togglei=&quot;&quot; , &quot;'&quot; , &quot; + attr + &quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;;
                if (t == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
                    $(header).attr(&quot; , &quot;'&quot; , &quot;data-wl-toggle&quot; , &quot;'&quot; , &quot;, 0);
                    $(i).removeClass(&quot; , &quot;'&quot; , &quot;fa-caret-down&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;fa-caret-up&quot; , &quot;'&quot; , &quot;);
                    $(selector).slideDown();
                } else {
                    $(header).attr(&quot; , &quot;'&quot; , &quot;data-wl-toggle&quot; , &quot;'&quot; , &quot;, 1);
                    $(i).addClass(&quot; , &quot;'&quot; , &quot;fa-caret-down&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;fa-caret-up&quot; , &quot;'&quot; , &quot;);
                    $(selector).slideUp();
                }
            }
            function staffRecordScrollIntoView() {
                let lowerCaseValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                const t = document.querySelector(&quot; , &quot;'&quot; , &quot;[data-wl-searchinput=&quot;staff&quot;]&quot; , &quot;'&quot; , &quot;);
                if (t != null) lowerCaseValue = t.value.toLowerCase();
                const emps = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-wl-empname]&quot; , &quot;'&quot; , &quot;);
                for (let i = 0, l = emps.length; i &lt; l; i++) {
                    const emp = emps[i];
                    const name = emp.getAttribute(&quot; , &quot;'&quot; , &quot;data-wl-empname&quot; , &quot;'&quot; , &quot;);
                    if (name.toLowerCase().includes(lowerCaseValue)) {
                        emp.scrollIntoView();
                        break;
                    }
                }
            }
            function getScrollPos() {
                const preSubmitList = document.getElementById(&quot; , &quot;'&quot; , &quot;preSubmitList&quot; , &quot;'&quot; , &quot;);
                if (preSubmitList != null) scrollPos = preSubmitList.scrollTop;
            }
            function setScrollPos() {
                const preSubmitList = document.getElementById(&quot; , &quot;'&quot; , &quot;preSubmitList&quot; , &quot;'&quot; , &quot;);
                if (preSubmitList != null) preSubmitList.scrollTop = scrollPos;
            }
            function validateHrs(e) {
                const t = e.target;
                const v = t.value;
                if (v &lt; 0) t.value = 0;
            }
            return {
                init: init
            }
        }());
    



    
        /*.badge-notify {
            background: red;
            position: relative !important;
            top: -12px;
            left: -15px;
            font-size-adjust: 0.4;
        }*/

        #xCentralisedSiteSelection {
            max-width: 250px;
            float: right;
            margin-right: 20px;
            z-index: 9999;
            position: absolute;
            top: 100px;
            right: 0;
        }
        /*the container must be positioned relative:*/
        .custom-select {
            position: relative;
            font-family: Arial;
        }

            .custom-select select {
                display: none; /*hide original SELECT element:*/
            }

        .select-selected {
            background-color: DodgerBlue;
        }
            /*style the arrow inside the select element:*/
            .select-selected:after {
                position: absolute;
                content: &quot;&quot;;
                top: 14px;
                right: 10px;
                width: 0;
                height: 0;
                border: 6px solid transparent;
                border-color: #fff transparent transparent transparent;
            }
            /*point the arrow upwards when the select box is open (active):*/
            .select-selected.select-arrow-active:after {
                border-color: transparent transparent #fff transparent;
                top: 7px;
            }
        /*style the items (options), including the selected item:*/
        .select-items div, .select-selected {
            color: #ffffff;
            padding: 8px 16px;
            border: 1px solid transparent;
            border-color: transparent transparent rgba(0, 0, 0, 0.1) transparent;
            cursor: pointer;
        }
        /*style items (options):*/
        .select-items {
            position: absolute;
            background-color: DodgerBlue;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 99;
        }
        /*hide the items when the select box is closed:*/
        .select-hide {
            display: none;
        }

        .select-items div:hover, .same-as-selected {
            background-color: rgba(0, 0, 0, 0.1);
        }
    
    
        function showhelp() {
            myWindow = window.open(&quot;/ShowHelp&quot;, &quot;_help&quot;, &quot;width=800, height=600&quot;);
        }
        let webAppMaster = function () {
            function registerBtnClocktimesEvent(btnID, sLink) {
                $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + btnID).click(function (e) {
                    window.open(sLink, &quot;ClockTimes&quot;);
                    this.blur();
                    return false;
                });
            }
            function registerHR(btnID, sLink) {
                $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + btnID).click(function (e) {
                    window.open(sLink, &quot;HR&quot;);
                    this.blur();
                    return false;
                });
            }
            function registerLogoff(btnID, changeID) {
                $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + btnID).click(function (e) {
                    const c = document.getElementById(changeID);
                    if (c != null) {
                        const changed = webAppUtils.parseBool(c.value);
                        if (changed) {
                            webAppUtils.msgShow(&quot; , &quot;'&quot; , &quot;Sign Out&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Changes that you made will not be saved.&lt;br>&lt;br>Do you still want to continue?&quot; , &quot;'&quot; , &quot;, webAppConfig.MsgBoxStyle.YesNo, webAppSession.logoutAllPages);
                        } else
                            webAppSession.logoutAllPages();
                    }
                    return false;
                });
                const a = document.getElementById(&quot; , &quot;'&quot; , &quot;chkAAAMode&quot; , &quot;'&quot; , &quot;);
                if (a != null) a.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () { webAppUtils.aaaModeAPI(this.checked); }, false);
                const d = document.getElementById(&quot; , &quot;'&quot; , &quot;chkDarkMode&quot; , &quot;'&quot; , &quot;);
                if (d != null) d.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () { webAppUtils.darkModeAPI(this.checked); }, false);
            }
            return {
                registerBtnClocktimesEvent: registerBtnClocktimesEvent,
                registerHR: registerHR,
                registerLogoff: registerLogoff
            }
        }();
    
    
        window.embeddedChatbotConfig = {
            chatbotId: &quot;DK_sXVyG_1_XSidG5zaMQ&quot;,
            domain: &quot;www.chatbase.co&quot;
        }
    
    
    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

    
    









//&lt;![CDATA[
var theForm = document.forms[&quot; , &quot;'&quot; , &quot;frmWebApp&quot; , &quot;'&quot; , &quot;];
if (!theForm) {
    theForm = document.frmWebApp;
}
function __doPostBack(eventTarget, eventArgument) {
    if (!theForm.onsubmit || (theForm.onsubmit() != false)) {
        theForm.__EVENTTARGET.value = eventTarget;
        theForm.__EVENTARGUMENT.value = eventArgument;
        theForm.submit();
    }
}
//]]&gt;







//&lt;![CDATA[
if (typeof(Sys) === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) throw new Error(&quot; , &quot;'&quot; , &quot;ASP.NET Ajax client-side framework failed to load.&quot; , &quot;'&quot; , &quot;);
//]]&gt;






//&lt;![CDATA[
function WebForm_OnSubmit() {
if (webAppSession.sessionExpired()) return false;webAppUtils.removeElement(webAppConfig.ID.ROSTERSTRIP_TABLE_BODY);$(&quot; , &quot;'&quot; , &quot;#MainContent_ucEditTimesheet_expEditTimesheet&quot; , &quot;'&quot; , &quot;).wijSaveState(&quot; , &quot;'&quot; , &quot;c1expander&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#ctl00_ctl17&quot; , &quot;'&quot; , &quot;).wijSaveState(&quot; , &quot;'&quot; , &quot;c1gridview&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onwijstatesaving&quot; , &quot;'&quot; , &quot;);
return true;
}
//]]&gt;




	
	

        
        
        
        
        
//&lt;![CDATA[
Sys.WebForms.PageRequestManager._initialize(&quot; , &quot;'&quot; , &quot;ctl00$ScriptManager1&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;frmWebApp&quot; , &quot;'&quot; , &quot;, [&quot; , &quot;'&quot; , &quot;tctl00$UpdatePanelMain&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;UpdatePanelMain&quot; , &quot;'&quot; , &quot;], [], [], 600, &quot; , &quot;'&quot; , &quot;ctl00&quot; , &quot;'&quot; , &quot;);
//]]&gt;


        
                
                
                
                
                    
                        
                            
                                
                                
                                
                            
                            
                        
                        
                            
                                
                                    Dashboard
                                
                                
                                    
                                
                                
                                    Rosters
                                
                                
                                    Timesheets
                                
                                
                                    Staff
                                
                                
                                    Clock Times
                                
                                
                                    Reports
                                
                                
                                    HR
                                
                            
                            
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            
                                            
                                        
                                    
                                
                                
                                    
                                        
                                        
                                    
                                    
                                        
                                        General
                                            
                                        
                                        Roster
                                        
                                        Timesheet
                                        
                                        
                                        
                                        
                                            Portal/Mobile App
                                           
                                            
                                            
                                                Leave
                                                Bulk Mobile Staff Clock
                                                Mobile Proximity Settings
                                                Portal Setup
                                            
                                        
                                        
                                        
                                            Payroll
                                        
                                            
                                            
                                                Pay Level
                                                Payroll Connection
                                                
                                                Payroll Mappings
                                                    
                                                
                                                
                                            
                                        
                                        
                                        
                                            Staff
                                                
                                            
                                            
                                                
                                                    Import
                                                        
                                                    
                                                    
                                                        Staff
                                                        Accrual Balances
                                                        Clock Times
                                                        Leave
                                                    
                                                
                                                
                                                Staff Locations
                                                Onboarding
                                            
                                        
                                        
                                        Departments
                                        
                                        Roles
                                        
                                        Qualifications
                                        
                                        Optional Allowances
                                        
                                        Colours
                                        
                                        Public Holidays
                                                 
                                        
                                            Audit
                                                
                                            
                                            
                                                Staff
                                                Roster
                                                Timesheet
                                            
                                        
                                           
                                        Security
                                        
                                        
                                            Demo
                                            
                                            
                                                Backup
                                                Restore
                                                Release IP Block
                                                Create Randomised Employees
                                            
                                                                                                         
                                    
                                
                                
                                    
                                        
                                            
                                        
                                    
                                
                                
                                    
                                        J
                                    
                                    
                                        
                                            
                                                J
                                            
                                            
                                                JerielQAjeriel_test@wageloch.com.au
                                            
                                            Manage your Wageloch User Account
                                            
                                        
                                        
                                            
                                                
                                                    Dark Mode
                                                    
                                                    
                                                
                                            
                                            
                                                
                                                    Colourblind Assist
                                                    
                                                    
                                                
                                            
                                        
                                        
                                            Sign Out
                                        
                                    
                                
                            
                        
                    
                
                
                    
                        
    Timesheet for the Week beginning February 9, 2026

                    
                    
                        
                        
	Central Payroll
	Central Site 1
	Central Site 2
	Central Site 3
	Central Site 4
	Stand Alone Site
	Wageloch Payroll


                    
                
                
                
                    
                    
                    
                    
                        
    
    


    .wl-grid-presubmit {
        height: calc(100vh - 280px - 45px);
    }

    .wl-rosterview-options input[type=&quot;radio&quot;]:checked + label {
        border-radius: 5px;
    }


	
    
		
        
            
            
			Barista
			Counter
			Kitchen
			Waitress
			Dishes
			Kitchenhand
			Chef
			Maintenance
			Marketing
			RSA
			RSP
			Manager
			Training
			Cook

		All selected (14)   Select all Barista Counter Kitchen Waitress Dishes Kitchenhand Chef Maintenance Marketing RSA RSP Manager Training Cook
        
    
	
    
        Pre-Submit
    
    
        
            
            
                
                
                    Save
                
                
                
                    Back
                
                
            
        
        
            
            
            
                Hide Empty Rows
            
            
                Hide Empty Columns
            
            
                
                    
                    
                        
                    
                
            
        
        
		
            
			
                            
                        
                            8.00
                        8.008.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        9.50
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        100.00
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        9.00
                            
                        
                            
                        
                            
                        
                            
                        8.00
                            
                        8.00
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        100.00
                            
                        100.00
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        
                            
                        100.00
                            
                        
                            
                        
                            
                        
                            
                        8.00
                            
                        
                            
                        100.00
                            
                        80.00
                            
                        100.00
                            
                        
                            
                                                              
                            Banner, Bruce
                            
                            
                            
                        
                            Baratheon, Joffrey
                            
                            
                            
                        
                            Behm, Mirian
                            
                            
                            
                        
                            Billings, Heidi
                            
                            
                            
                        
                            Blanc, Koriks
                            
                            
                            
                        
                            Bouldin, Karren
                            
                            
                            
                        
                            Brogan, Neta
                            
                            
                            
                        
                            Cardamone, Tomas
                            
                            
                            
                        
                            Cashman, Ivonne
                            
                            
                            
                        
                            Chambermaid, King Prince
                            
                            
                            
                        
                            Cluggins, Fartrell
                            
                            
                            
                        
                            Corter, Elipses
                            
                            
                            
                        
                            Dookmarriot, L&quot; , &quot;'&quot; , &quot;Carpetron
                            
                            
                            
                        
                            Doughty, Luanne
                            
                            
                            
                        
                            Drake, Bobby
                            
                            
                            
                        
                            Drogo, Khal
                            
                            
                            
                        
                            Earhart, Edie
                            
                            
                            
                        
                            Employee333, Employee333
                            
                            
                            
                        
                            Employee500, Employee500
                            
                            
                            
                        
                            Frost, Emma
                            
                            
                            
                        
                            Funyuns, Bismo
                            
                            
                            
                        
                            Grey, Jean
                            
                            
                            
                        
                            Gulick, Cliff
                            
                            
                            
                        
                            Hemingway, Morris
                            
                            
                            
                        
                            Ikerd, Na
                            
                            
                            
                        
                            Ingles, Neta
                            
                            
                            
                        
                            Jilliumz, Leoz Maxwell
                            
                            
                            
                        
                            Jtc, Nob
                            
                            
                            
                        
                            Kent, Clark
                            
                            
                            
                        
                            Kopczynski, Carma
                            
                            
                            
                        
                            Lannister, Cersei
                            
                            
                            
                        
                            Lannister, Jamie
                            
                            
                            
                        
                            Lannister, Tyrion
                            
                            
                            
                        
                            LeBeau, Remy
                            
                            
                            
                        
                            Lewith, Torque
                            
                            
                            
                        
                            L&quot; , &quot;'&quot; , &quot;Goodling-Splatt, Swirvithan
                            
                            
                            
                        
                            Littler, Porsha
                            
                            
                            
                        
                            Looper, Elane
                            
                            
                            
                        
                            Machine, Age
                            
                            
                            
                        
                            Mangino, Glayds
                            
                            
                            
                        
                            Marcotte, Esperanza
                            
                            
                            
                        
                            Marte, Cliff
                            
                            
                            
                        
                            McCringleberry, Hingle
                            
                            
                            
                        
                            moz, zom
                            
                            
                            
                        
                            Munroe, Ororo
                            
                            
                            
                        
                            Oll, Red
                            
                            
                            
                        
                            Otte;:, Cathryn
                            
                            
                            
                        
                            Pineo, Aide
                            
                            
                            
                        
                            Pride, Kitty
                            
                            
                            
                        
                            Queen, Oliver
                            
                            
                            
                        
                            Rasputin, Piotr
                            
                            
                            
                        
                            Red, Oll
                            
                            
                            
                        
                            Roboclick, Stumptavian
                            
                            
                            
                        
                            Shower-Handel, Davoin
                            
                            
                            
                        
                            Snow, Jon
                            
                            
                            
                        
                            Soucie, Brian
                            
                            
                            
                        
                            Staffsn01, Stafffn01
                            
                            
                            
                        
                            staffsname, stafffname01
                            
                            
                            
                        
                            Staffsurname1, Staffname1
                            
                            
                            
                        
                            Stark, Arya
                            
                            
                            
                        
                            Stark, Sansa
                            
                            
                            
                        
                            Stock, Tomasa
                            
                            
                            
                        
                            Targaryen, Daenerys
                            
                            
                            
                        
                            Test123000, Test123000
                            
                            
                            
                        
                            test214214, test214214
                            
                            
                            
                        
                            Testemp00001, Testemp00001
                            
                            
                            
                        
                            Testemp00003, Testemp00003
                            
                            
                            
                        
                            Testemp00004, Testemp00004
                            
                            
                            
                        
                            Testemp00005, Testemp00005
                            
                            
                            
                        
                            Testemp00006, Testemp00006
                            
                            
                            
                        
                            Testemp00007, Testemp00007
                            
                            
                            
                        
                            Testemp00008, Testemp00008
                            
                            
                            
                        
                            Testemp00009, Testemp00009
                            
                            
                            
                        
                            Testemp00010, Testemp00010
                            
                            
                            
                        
                            Testemp00011, Testemp00011
                            
                            
                            
                        
                            Testemp00012, Testemp00012
                            
                            
                            
                        
                            Testemp00013, Testemp00013
                            
                            
                            
                        
                            Testemp00014, Testemp00014
                            
                            
                            
                        
                            Testemp00015, Testemp00015
                            
                            
                            
                        
                            Testemp00016, Testemp00016
                            
                            
                            
                        
                            Testemp00017, Testemp00017
                            
                            
                            
                        
                            Testemp00018, Testemp00018
                            
                            
                            
                        
                            Testemp00019, Testemp00019
                            
                            
                            
                        
                            Testemp00020, Testemp00020
                            
                            
                            
                        
                            Testemp00021, Testemp00021
                            
                            
                            
                        
                            Testemp00022, Testemp00022
                            
                            
                            
                        
                            Testemp00023, Testemp00023
                            
                            
                            
                        
                            Testemp00024, Testemp00024
                            
                            
                            
                        
                            Testemp00025, Testemp00025
                            
                            
                            
                        
                            Testemp00026, Testemp00026
                            
                            
                            
                        
                            Testemp00027, Testemp00027
                            
                            
                            
                        
                            Testemp00028, Testemp00028
                            
                            
                            
                        
                            Testemp00029, Testemp00029
                            
                            
                            
                        
                            Testemp00030, Testemp00030
                            
                            
                            
                        
                            Testemp00031, Testemp00031
                            
                            
                            
                        
                            Testemp00032, Testemp00032
                            
                            
                            
                        
                            Testemp00033, Testemp00033
                            
                            
                            
                        
                            Testemp00034, Testemp00034
                            
                            
                            
                        
                            Testemp00035, Testemp00035
                            
                            
                            
                        
                            Testemp00036, Testemp00036
                            
                            
                            
                        
                            Testemp00037, Testemp00037
                            
                            
                            
                        
                            Testemp00038, Testemp00038
                            
                            
                            
                        
                            Testemp00039, Testemp00039
                            
                            
                            
                        
                            Testemp00040, Testemp00040
                            
                            
                            
                        
                            Testemp00041, Testemp00041
                            
                            
                            
                        
                            Testemp00042, Testemp00042
                            
                            
                            
                        
                            Testemp00043, Testemp00043
                            
                            
                            
                        
                            Testemp00044, Testemp00044
                            
                            
                            
                        
                            Testemp00045, Testemp00045
                            
                            
                            
                        
                            Testemp00046, Testemp00046
                            
                            
                            
                        
                            Testemp00047, Testemp00047
                            
                            
                            
                        
                            Testemp00048, Testemp00048
                            
                            
                            
                        
                            Testemp00049, Testemp00049
                            
                            
                            
                        
                            Testemp00050, Testemp00050
                            
                            
                            
                        
                            TESTQA1084129745, TESTQA1084129745
                            
                            
                            
                        
                            TestQA108812742, TestQA108812742
                            
                            
                            
                        
                            TESTQA120983588, TESTQA120983588
                            
                            
                            
                        
                            Twitchell, Season
                            
                            
                            
                        
                            Vanderwal, Eula
                            
                            
                            
                        
                            Vanhoy, Mamie
                            
                            
                            
                        
                            Wagner, Kurt
                            
                            
                            
                        
                            Wayne, Bruce
                            
                            
                            
                        
                            Worthington III, Warren
                            
                            
                            
                        
                            Wrede, Heidi
                            
                            
                            
                        
                            Youssef, Rikki
                            
                            
                            
                         Total HoursTotal WorkedOrdM-FSatSunPub HolCas M-FCas SatCas SunCas L3 SunCas Pub HolCas Bef 7aCas Aft 10pBef 7aAft 10pO/T 1.5O/T 1.75O/T 2.0L2 M-FL2 SATL2 SUNL2 PHRESP M-FRESP SATRESP SUNRESP PHSUPERVISORMANAGERPHNoWorkANNUAL LVESICK LEAVEUNPAID LEAVETOIL PAIDWORKCOVERLSLTOIL ACCRUEDAlt HolidayStaff Name
		
        
	
        
    








    
    
    
    
    
    

                    
                    
                    
                    
                    
                
                
	
                    
                        
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                            
                        
                    
                

            
        
	
                
                    
                
            

        
	

    


//&lt;![CDATA[
webAppRosterStrips.myInit({parentID: &quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_ucRosterStrip_divStrips&quot; , &quot;'&quot; , &quot;,rowHeight: &quot; , &quot;'&quot; , &quot;40&quot; , &quot;'&quot; , &quot;,rosterByEmployee: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,createStaffBtn: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,btnStaffID: &quot; , &quot;'&quot; , &quot;ctl00$MainContent$ucEditTimesheet$ctl01$ucRosterStrip$btnStaff&quot; , &quot;'&quot; , &quot;,createStaffStrip: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,refreshAlert: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,isTimesheet: &quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;,flipTimes: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,showRatio: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,minShiftMins: &quot; , &quot;'&quot; , &quot;30&quot; , &quot;'&quot; , &quot;,labelTotalHrs: &quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_lblTotalHrs&quot; , &quot;'&quot; , &quot;,labelTotalCost: &quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_lblTotalCost&quot; , &quot;'&quot; , &quot;,isFromGrid: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,useDeptColors: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,useOptAllow: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,SnapPaid: &quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;,OnlyShowWorkedHours: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,useBrk: &quot; , &quot;'&quot; , &quot;False&quot; , &quot;'&quot; , &quot;,brkMins: 0,insertBrkWhenLongerThan: 0,SplitBrkDifferential: 90,loadImg: true});webAppAPI.init(&quot; , &quot;'&quot; , &quot;https://test.wageloch.com.au/WebAppApi.aspx/&quot; , &quot;'&quot; , &quot;);initSportalApi();webAppUtils.myInit(&quot; , &quot;'&quot; , &quot;aTimesheets&quot; , &quot;'&quot; , &quot;);webAppSession.sessionExpireTimer(15, &quot; , &quot;'&quot; , &quot;H0KhMU1aTk6QjM1HS1dnMvq4eiQp&quot; , &quot;'&quot; , &quot;, false);webAppUtils.resetTimers(5);webAppMaster.registerBtnClocktimesEvent(&quot; , &quot;'&quot; , &quot;btnClocktimes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/clocktimelist&quot; , &quot;'&quot; , &quot;);webAppMaster.registerHR(&quot; , &quot;'&quot; , &quot;aHr&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/HRconfig&quot; , &quot;'&quot; , &quot;);webAppMaster.registerLogoff(&quot; , &quot;'&quot; , &quot;aLogout&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;hfChanged&quot; , &quot;'&quot; , &quot;);webAppUtils.removeQueryString();webAppUtils.getGlobalAlertCntAPI(&quot; , &quot;'&quot; , &quot;_tst_138051_5&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;bea3891d-d0dc-4382-bc11-f43b4b565b85&quot; , &quot;'&quot; , &quot;);webAppRosterStrips.getRosterAlertCnt(true);webAppUtils.setupDaysScroll(&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_rbDays&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_btnRbDaysLeft&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_btnRbDaysRight&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_ctl01_hfScrollLeft&quot; , &quot;'&quot; , &quot;);webAppRosterStrips.calcCosting();$.disposeCallbackDatas[&quot; , &quot;'&quot; , &quot;MainContent_ucEditTimesheet_expEditTimesheet&quot; , &quot;'&quot; , &quot;]=&quot; , &quot;'&quot; , &quot;c1expander&quot; , &quot;'&quot; , &quot;;$.disposeCallbackDatas[&quot; , &quot;'&quot; , &quot;ctl00_ctl17&quot; , &quot;'&quot; , &quot;]=&quot; , &quot;'&quot; , &quot;c1gridview&quot; , &quot;'&quot; , &quot;;
WebForm_InitCallback();Sys.Application.add_init(function() {
    jQuery(&quot;#MainContent_ucEditTimesheet_expEditTimesheet&quot;).c1expander(wijmoASPNetParseOptions({&quot;allowExpand&quot;: false, &quot;disabledState&quot;: false}));
});
Sys.Application.add_init(function() {
    $create(Sys.UI._UpdateProgress, {&quot;associatedUpdatePanelId&quot;:null,&quot;displayAfter&quot;:1,&quot;dynamicLayout&quot;:true}, null, null, $get(&quot;UpdatePp1&quot;));
});
Sys.Application.add_init(function() {
    jQuery(&quot;#ctl00_ctl17&quot;).c1gridview(wijmoASPNetParseOptions({&quot;culture&quot;: &quot;en-AU&quot;, &quot;pageIndex&quot;: 0, &quot;columns&quot;: [], &quot;pageCount&quot;: -1, &quot;showClientSelectionOnRender&quot;: true, &quot;innerState&quot;: {&quot;_pbRef&quot;: &quot;__doPostBack(&quot; , &quot;'&quot; , &quot;ctl00$ctl17&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;{0}&quot; , &quot;'&quot; , &quot;)&quot;, &quot;_cid&quot;: &quot;ctl00_ctl17&quot;, &quot;dbf&quot;: false, &quot;_uid&quot;: &quot;ctl00$ctl17&quot;, &quot;_bodyEditIndex&quot;: -1, &quot;scrollingState&quot;: {&quot;x&quot;:null,&quot;y&quot;:null,&quot;index&quot;:0}, &quot;_root&quot;: 1, &quot;_culture&quot;: &quot;en-AU&quot;, &quot;_bodySelectedIndex&quot;: -1, &quot;_dc&quot;: &quot;aspNetDisabled&quot;}, &quot;disabledState&quot;: false}));
});
//]]&gt;


    
        
            
                © 2024 - Wageloch
                
                
            
            
            
            
                window.dataLayer = window.dataLayer || [];
                function gtag() { dataLayer.push(arguments); }
                gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());

                gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;G-T6DX3BN0JJ&quot; , &quot;'&quot; , &quot;);
            

            
                var x, i, j, selElmnt, a, b, c;
                /*look for any elements with the class &quot;custom-select&quot;:*/
                x = document.getElementsByClassName(&quot;custom-select&quot;);
                for (i = 0; i &lt; x.length; i++) {
                    selElmnt = x[i].getElementsByTagName(&quot;select&quot;)[0];
                    /*for each element, create a new DIV that will act as the selected item:*/
                    a = document.createElement(&quot;DIV&quot;);
                    a.setAttribute(&quot;class&quot;, &quot;select-selected&quot;);
                    a.innerHTML = selElmnt.options[selElmnt.selectedIndex].innerHTML;
                    x[i].appendChild(a);
                    /*for each element, create a new DIV that will contain the option list:*/
                    b = document.createElement(&quot;DIV&quot;);
                    b.setAttribute(&quot;class&quot;, &quot;select-items select-hide&quot;);
                    for (j = 1; j &lt; selElmnt.length; j++) {
                        /*for each option in the original select element,
                        create a new DIV that will act as an option item:*/
                        c = document.createElement(&quot;DIV&quot;);
                        c.innerHTML = selElmnt.options[j].innerHTML;
                        c.addEventListener(&quot;click&quot;, function (e) {
                            /*when an item is clicked, update the original select box,
                            and the selected item:*/
                            var y, i, k, s, h;
                            s = this.parentNode.parentNode.getElementsByTagName(&quot;select&quot;)[0];
                            h = this.parentNode.previousSibling;
                            for (i = 0; i &lt; s.length; i++) {
                                if (s.options[i].innerHTML == this.innerHTML) {
                                    s.selectedIndex = i;
                                    h.innerHTML = this.innerHTML;
                                    y = this.parentNode.getElementsByClassName(&quot;same-as-selected&quot;);
                                    for (k = 0; k &lt; y.length; k++) {
                                        y[k].removeAttribute(&quot;class&quot;);
                                    }
                                    this.setAttribute(&quot;class&quot;, &quot;same-as-selected&quot;);
                                    break;
                                }
                            }
                            h.click();
                        });
                        b.appendChild(c);
                    }
                    x[i].appendChild(b);
                    a.addEventListener(&quot;click&quot;, function (e) {
                        /*when the select box is clicked, close any other select boxes,
                        and open/close the current select box:*/
                        e.stopPropagation();
                        closeAllSelect(this);
                        this.nextSibling.classList.toggle(&quot;select-hide&quot;);
                        this.classList.toggle(&quot;select-arrow-active&quot;);
                    });
                }
                function closeAllSelect(elmnt) {
                    /*a function that will close all select boxes in the document,
                    except the current select box:*/
                    var x, y, i, arrNo = [];
                    x = document.getElementsByClassName(&quot;select-items&quot;);
                    y = document.getElementsByClassName(&quot;select-selected&quot;);
                    for (i = 0; i &lt; y.length; i++) {
                        if (elmnt == y[i]) {
                            arrNo.push(i)
                        } else {
                            y[i].classList.remove(&quot;select-arrow-active&quot;);
                        }
                    }
                    for (i = 0; i &lt; x.length; i++) {
                        if (arrNo.indexOf(i)) {
                            x[i].classList.add(&quot;select-hide&quot;);
                        }
                    }
                }
                /*if the user clicks anywhere outside the select box,
                then close all select boxes:*/
                document.addEventListener(&quot;click&quot;, closeAllSelect);
            
        
    
    
        
            
                Cookies Required
                Cookies are not enabled on your browser. Please enable cookies in your browser settings to continue.
            
        
    



#chatbase-bubble-button,
#chatbase-bubble-button * {
 margin: 0;
 padding: 0;
 box-sizing: border-box;
}
	✕Hi, I&quot; , &quot;'&quot; , &quot;m Hugo. How can I help you today?/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]&quot;))]</value>
      <webElementGuid>351271b2-f0df-4e59-bfee-9ea5c678bbb2</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
